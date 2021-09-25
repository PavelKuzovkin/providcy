# -*- coding: utf-8 -*-
"""
Descrition
"""
__author__ = 'maxdob'
__date__ = '25.09.21'
__time__ = '23:54'
__version__ = '0.1'

# from threading import Thread
from exonum_client.api import ServiceApi
from .BaseService import BaseService
from settings import Exonum

class InsurerService(BaseService):

    track_entities = ["loan_request", "loan_orders_list"]

    def __init__(self, name: str):
        super().__init__(name, self.track_entities)
        self._name = name
        self._track_entities = self.track_entities
        self.__service = ServiceApi(service_name = Exonum._service_name, hostname = Exonum._host,
                                    port = Exonum._port, schema=Exonum._schema)