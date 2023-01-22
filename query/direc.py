import os,sys

this = os.path.dirname(os.path.abspath(__file__))
ok = os.path.dirname(this)
sys.path.insert(0,ok)

from definition import *
