import ipaddress
import socket

from bitstruct import *
from recordclass import RecordClass


class DnsHeader(RecordClass):
    id: int
    response: int
    opcode: int
    authoritative_answer: int
    truncated_message: int
    recursion_desired: int
    recursion_available: int
    z: int
    rescode: int
    questions: int
    answers: int
    authoritative_entries: int
    resource_entries: int


HEADER_BINARY_STRUCT = ">u16>u1>u4>u1>u1>u1>u1>u3>u4>u16>u16>u16>u16"


class DnsQuestion(RecordClass):
    name: str
    qtype: int


class DnsRecord_A(RecordClass):
    domain: str
    addr: str
    ttl: int


class DnsRecord_AAAA(RecordClass):
    domain: str
    addr: str
    ttl: int


class DnsRecord_CNAME(RecordClass):
    domain: str
    host: str
    ttl: int


class DnsRecord_NS(RecordClass):
    domain: str
    host: str
    ttl: int


class DnsRecord_MX(RecordClass):
    domain: str
    addr: str
    ttl: int
    priority: int


class DnsRecord_UNKNOWN(RecordClass):
    domain: str
    qtype: int
    data_len: int
    ttl: int


class DnsPacket(RecordClass):
    header: DnsHeader
    questions: list
    answers: list
    authorities: list
    resources: list


UNKNOWN = 0
A = 1
NS = 2
CNAME = 5
MX = 15
AAAA = 28

NOERROR = 0
FORMERR = 1
SERVFAIL = 2
NXDOMAIN = 3
NOTIMP = 4
REFUSED = 5


def main(host, port):
    server_address = (host, port)
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)  # DNS es UDP
    sock.bind(server_address)
    while True:
        handle_query(sock)

buffer_size = 512

def handle_query(sock):
    buf, address = sock.recvfrom(buffer_size)
    request = parse_packet(buf)
    packet = DnsPacket(DnsHeader(request.header.id, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0), [], [], [], [])
    question = request.questions.pop()
    if question:
        result = recursive_lookup(question.name, question.qtype)
        if result:
            packet.questions.append(question)
            packet.header.rescode = result.header.rescode
            packet.answers = result.answers
            packet.authorities = result.authorities
            packet.resources = result.resources
        else:
            packet.header.rescode = SERVFAIL
    else:
        packet.header.rescode = FORMERR

    out_buf = write_packet(packet)
    sock.sendto(out_buf, address)


def recursive_lookup(qname, qtype):
    #  partiremos con a.root-servers.net
    ns = "198.41.0.4"
    while True:
        print("intentamos búsqueda de {} {} con el ns {}".format(qtype, qname, ns))
        ns_copy = ns
        server = (str(ns_copy), 53)
        response = lookup(qname, qtype, server)
        if response.answers and response.header.rescode == NOERROR:
            return response

        if response.header.rescode == NXDOMAIN:
            return response

        new_ns = get_resolved_ns(response, qname)
        if new_ns:
            ns = new_ns
            continue

        new_ns_name = get_unresolved_ns(response, qname)
        if not new_ns_name:
            return response

        recursive_response = recursive_lookup(new_ns_name, A)

        new_ns = get_random_a(recursive_response)
        if new_ns:
            ns = new_ns
        else:
            return response


def lookup(qname, qtype, server):
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)  # DNS es UDP
    sock.bind(("0.0.0.0", 43210))
    packet = DnsPacket(DnsHeader(6666, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0), [DnsQuestion(qname, qtype)], [], [],
                       [])
    req_buffer = write_packet(packet)
    sock.sendto(req_buffer, server)
    buf, _ = sock.recvfrom(512)
    return parse_packet(buf)


def get_random_a(dns_packet):
    result = [a.addr for a in dns_packet.answers if isinstance(a, DnsRecord_A)]
    if result:
        return str(result[0])
    return None


def get_resolved_ns(dns_packet, qname):
    ns = get_ns(dns_packet, qname)
    for host in [host for (_, host) in ns]:
        r = [r.addr for r in dns_packet.resources if isinstance(r, DnsRecord_A) and r.domain == host]
        if r:
            return r[0]
    return None

def get_unresolved_ns(dns_packet, qname):
    ns = get_ns(dns_packet, qname)
    if not ns:
        return None
    (_, host) = ns[0]
    return host


def get_ns(dns_packet, qname):
    ns = [(a.domain, a.host) for a in dns_packet.authorities if isinstance(a, DnsRecord_NS)]
    return [(domain, host) for (domain, host) in ns if qname.endswith(domain)]


def write_packet(packet):
    packet.header.questions = len(packet.questions)
    packet.header.answers = len(packet.answers)
    packet.header.authoritative_entries = len(packet.authorities)
    packet.header.resource_entries = len(packet.resources)

    buf = bytearray()
    header = packet.header
    buf.extend(pack('>u16', header.id))
    buf.extend(pack('>u1u4u1u1u1u1u3u4', header.response, header.opcode, header.authoritative_answer,
                    header.truncated_message, header.recursion_desired, header.recursion_available,
                    header.z, header.rescode))
    buf.extend(pack('>u16', header.questions))
    buf.extend(pack('>u16', header.answers))
    buf.extend(pack('>u16', header.authoritative_entries))
    buf.extend(pack('>u16', header.resource_entries))

    for q in packet.questions:
        buf.extend(write_question(q))

    for a in packet.answers:
        buf.extend(write_dns_record(a))

    for a in packet.authorities:
        buf.extend(write_dns_record(a))

    for r in packet.resources:
        buf.extend(write_dns_record(r))

    return buf


def write_question(question):
    buf = bytearray()
    buf.extend(write_qname(question.name))
    buf.extend(pack(">u16", question.qtype))
    buf.extend(pack(">u16", 1))
    return buf


def write_qname(name):
    buf = bytearray()
    labels = name.split('.')
    for label in labels:
        l = len(label)
        if l > 0x34:
            return None
        buf.append(l)
        for b in label:
            buf.append(ord(b))
    buf.append(0)
    return buf


def write_dns_record(record):
    buf = bytearray()
    if isinstance(record, DnsRecord_A):
        buf.extend(write_qname(record.domain))
        buf.extend(pack(">u16", A))
        buf.extend(pack(">u16", 1))
        buf.extend(pack(">u32", record.ttl))
        buf.extend(pack(">u16", 4))
        buf.extend(pack(">u32", int(record.addr)))
        return buf
    elif isinstance(record, DnsRecord_AAAA):
        buf.extend(write_qname(record.domain))
        buf.extend(pack(">u16", AAAA))
        buf.extend(pack(">u16", 1))
        buf.extend(pack(">u32", record.ttl))
        buf.extend(pack(">u16", 16))
        for octet in record.addr.exploded.split(':'):
            buf.extend(pack(">u16", int(octet, 16)))
        return buf
    elif isinstance(record, DnsRecord_NS):
        buf.extend(write_qname(record.domain))
        buf.extend(pack(">u16", NS))
        buf.extend(pack(">u16", 1))
        buf.extend(pack(">u32", record.ttl))
        pos = len(buf)
        buf.extend(pack(">u16", 0))
        buf.extend(write_qname(record.host))
        pos_1 = len(buf)
        size = pos_1 - (pos+2)
        buf[pos] = size >> 8
        buf[pos+1] = (size & 0xFF)
        return buf
    elif isinstance(record, DnsRecord_CNAME):
        buf.extend(write_qname(record.domain))
        buf.extend(pack(">u16", CNAME))
        buf.extend(pack(">u16", 1))
        buf.extend(pack(">u32", record.ttl))
        pos = len(buf)
        buf.extend(pack(">u16", 0))
        buf.extend(write_qname(record.host))
        pos_1 = len(buf)
        size = pos_1 - (pos+2)
        buf[pos] = size >> 8
        buf[pos+1] = (size & 0xFF)
        return buf
    elif isinstance(record, DnsRecord_MX):
        buf.extend(write_qname(record.domain))
        buf.extend(pack(">u16", MX))
        buf.extend(pack(">u16", 1))
        buf.extend(pack(">u32", record.ttl))
        pos = len(buf)
        buf.extend(pack(">u16", 0))
        buf.extend(pack(">u16", record.priority))
        buf.extend(write_qname(record.host))
        pos_1 = len(buf)
        size = pos_1 - (pos+2)
        buf[pos] = size >> 8
        buf[pos+1] = (size & 0xFF)
        return buf
    else:
        print("Skipping record: {:?}", record)
        return buf


def parse_packet(buf):
    unpacked = unpack(HEADER_BINARY_STRUCT, buf)
    header = DnsHeader(*unpacked)
    pos = 12
    pos, questions = parse_questions(header, pos, buf)
    pos, answers = parse_answers(header, pos, buf)
    pos, authorities = parse_authorities(header, pos, buf)
    pos, resources = parse_resources(header, pos, buf)
    return DnsPacket(header, questions, answers, authorities, resources)


def parse_elements(limit, pos, buf, parser):
    result = []
    for i in range(0, limit):
        pos, e = parser(pos, buf)
        result.append(e)
    return pos, result

def parse_questions(header, pos, buf):
    return parse_elements(header.questions, pos, buf, parse_question)


def parse_answers(header, pos, buf):
    return parse_elements(header.answers, pos, buf, parse_dns_record)


def parse_authorities(header, pos, buf):
    return parse_elements(header.authoritative_entries, pos, buf, parse_dns_record)

def parse_resources(header, pos, buf):
    return parse_elements(header.resource_entries, pos, buf, parse_dns_record)


def parse_question(pos, buf):
    pos, name = parse_qname(pos, buf)
    pos, qtype = parse_u16(pos, buf)
    pos, _ = parse_u16(pos, buf)
    return pos, DnsQuestion(name, qtype)


def parse_dns_record(pos, buf):
    pos, domain = parse_qname(pos, buf)
    pos, qtype = parse_u16(pos, buf)
    pos, _ = parse_u16(pos, buf)
    pos, ttl = parse_u32(pos, buf)
    pos, data_len = parse_u16(pos, buf)
    if qtype == A:
        pos, raw_addr = parse_u32(pos, buf)
        addr = ipaddress.ip_address(raw_addr)
        return pos, DnsRecord_A(domain, addr, ttl)
    elif qtype == AAAA:
        pos, raw_addr1 = parse_u32(pos, buf)
        pos, raw_addr2 = parse_u32(pos, buf)
        pos, raw_addr3 = parse_u32(pos, buf)
        pos, raw_addr4 = parse_u32(pos, buf)
        x = "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}".format((raw_addr1 >> 16) & 0xFFFF, raw_addr1 & 0xFFFF,
                                                             (raw_addr2 >> 16) & 0xFFFF, raw_addr2 & 0xFFFF,
                                                             (raw_addr3 >> 16) & 0xFFFF, raw_addr3 & 0xFFFF,
                                                             (raw_addr4 >> 16) & 0xFFFF, raw_addr4 & 0xFFFF,
                                                             )
        addr = ipaddress.ip_address(x)
        return pos, DnsRecord_AAAA(domain, addr, ttl)
    elif qtype == NS:
        pos, host = parse_qname(pos, buf)
        return pos, DnsRecord_NS(domain, host, ttl)
    elif qtype == CNAME:
        pos, host = parse_qname(pos, buf)
        return pos, DnsRecord_CNAME(domain, host, ttl)
    elif qtype == MX:
        pos, priority = parse_u16(pos, buf)
        pos, host = parse_qname(pos, buf)
        return pos, DnsRecord_MX(domain, host, ttl, priority)
    else:
        pos += data_len
        return pos, DnsRecord_UNKNOWN(domain, qtype, data_len, ttl)


def parse_u16(pos, buf):
    return pos + 2, unpack("u16", buf[pos:pos + 2])[0]


def parse_u32(pos, buf):
    return pos + 4, unpack("u32", buf[pos:pos + 4])[0]


def parse_qname(pos, buf):
    parts = []
    max_jumps = 5
    jumps_performed = 0
    p = pos
    l = int(buf[p])
    while l > 0 and jumps_performed <= max_jumps:
        if (l & 0xC0) == 0XC0:
            if jumps_performed == 0:
                pos = p + 2
            b2 = int(buf[p + 1])
            offset = ((l ^ 0xC0) << 8) | b2
            p = offset
            jumps_performed += 1
        else:
            p += 1
            str_buf = str(buf[p:p + l].decode("utf-8"))
            parts.append(str_buf.lower())
            p += l
        l = int(buf[p])
    if jumps_performed > max_jumps:
        return pos, None
    if jumps_performed == 0:
        pos = p + 1
    return pos, str(str(".").join(parts))


if __name__ == '__main__':
    main('localhost', 2053)
