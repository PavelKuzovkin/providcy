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
    track_entities = ["insurance",]

    def __init__(self, name: str):
        super().__init__(name, self.track_entities)
        # self.__service.service_endpoint()