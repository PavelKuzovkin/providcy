# -*- coding: utf-8 -*-
"""
Descrition
"""
__author__ = 'maxdob'
__date__ = '25.09.21'
__time__ = '23:02'
__version__ = '0.1'

__all__ = ['bank','insurer']

from . import BankService
from . import InsurerService

bank = BankService.BankService
insurer = InsurerService.InsurerService