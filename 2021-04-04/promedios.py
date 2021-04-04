import random
from multiprocessing import shared_memory, Process
from multiprocessing.managers import SharedMemoryManager

def calc_promedio(notas, pos, curso):
    n = len(curso)
    s = sum(curso)
    notas[pos] = s / n 

def promedios():
    curso1 = [random.randint(1,7) for _ in range(30)]
    curso2 = [random.randint(1,7) for _ in range(30)]
    curso3 = [random.randint(1,7) for _ in range(30)]
    curso4 = [random.randint(1,7) for _ in range(30)]

    with SharedMemoryManager() as smm:
        notas = smm.ShareableList([0, 0, 0, 0])
        process = []
        process.append(Process(target=calc_promedio, args=(notas, 0, curso1)))
        process.append(Process(target=calc_promedio, args=(notas, 1, curso2)))
        process.append(Process(target=calc_promedio, args=(notas, 2, curso3)))
        process.append(Process(target=calc_promedio, args=(notas, 3, curso4)))
        for p in process:
            p.start()
        for p in process:
            p.join()
        for i, n in enumerate(notas):
            print(f"promedio curso {i+1} es {n:.2f}")

if __name__ == "__main__":
    promedios()
