# -*- coding: utf-8 -*-
"""
Descrition
"""
__author__ = 'maxdob'
__date__ = '25.09.21'
__time__ = '23:54'
__version__ = '0.1'

from .BaseService import BaseService

class BankService(BaseService):

    __key_pair = {"insurance":("insurer","policy_number")}
    track_entities = ["insurance",]
    #insurer + policy_number

    def __init__(self, name: str):
        super().__init__(name, self.track_entities)

    def sending(self):
        pass