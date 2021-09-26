# -*- coding: utf-8 -*-
"""
Descrition
"""
__author__ = 'maxdob'
__date__ = '25.09.21'
__time__ = '23:54'
__version__ = '0.1'

from .BaseService import BaseService


class InsurerService(BaseService):
    key_pair = {"loan_request": ("bank", "request_number"), "loan_order": ("bank", "order_number")}
    track_entities = ["loan_request", "loan_order"]
    queue = "insurer"

    def __init__(self, name: str):
        super().__init__(name, self.track_entities, self.key_pair)
