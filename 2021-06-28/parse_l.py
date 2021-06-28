# parser 

# digito excluyendo cero = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" 
def digito_excluyendo_cero(s):
   return s in ['1', '2', '3', '4', '5', '6', '7', '8', '9']

# digito = "0" | digito excluyendo cero 
def digito(s):
   return s == '0' or digito_excluyendo_cero(s)

# numero = digito excluyendo cero { digito } 
def numero(s):
   if len(s) == 0: return False
   if not digito_excluyendo_cero(s[0]): return False
   if len(s) > 1:
      for c in s[1:]: 
         if not digito(c): return False
   return True

# adicion = numero | numero, "+", expresion
def adicion(s):
   if not '+' in s:
      return numero(s)

   parts = s.split('+', 2)
   if not numero(parts[0]): return False
   if not expresion(parts[1]): return False
   return True


def igualdad(s):
   if '=' not in s:
      return False
   parts = s.split('=')
   if len(parts) != 2:
      return False
   if not adicion(parts[0]): return False
   if not adicion(parts[1]): return False
   return True

# expresion = adicion | igualdad 
def expresion(s):
   if adicion(s):
      return True
   if igualdad(s):
      return True
   return False


assert(expresion("2+4"))
assert(expresion("2+4+10"))
assert(expresion("2+4+66=8"))
assert(not expresion("2+4=4=44"))
assert(expresion("23+4=55"))
