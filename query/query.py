import time
from direc import *
from data import *

db=database()
conn=db.connect()
cur=conn.cursor()
a = open(input("input name of sql file:\n")+'.sql','r').read()
print('read')
cur.execute(a)
print('executed')
conn.commit()
print('success')
time.sleep(2)

