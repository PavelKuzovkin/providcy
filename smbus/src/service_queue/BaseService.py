# -*- coding: utf-8 -*-
"""
Descrition
"""
__author__ = 'maxdob'
__date__ = '25.09.21'
__time__ = '21:32'
__version__ = '0.1'

import time
from exonum_client.api import ServiceApi
from settings import Exonum
from notifier.mail_notifier import MailNotifier


def sending(sub, mess) -> bool:
    return MailNotifier(sub, mess).send()


class BaseService:
    __processed_yet = []

    def __init__(self, name: str, track_entities: list, key_pair: dict):
        self._name = name
        self._track_entities = track_entities
        self._key_pair = key_pair
        self.__service = ServiceApi(service_name=Exonum._service_name, hostname=Exonum._host,
                                    port=Exonum._port, schema=Exonum._schema)

    def _run(self):
        while True:
            self.__main_loop()
            time.sleep(30)

    def update(self):
        self.__main_loop()

    def __main_loop(self):
        for entity in self._track_entities:
            try:
                data = self.__service.get_service("v1/{0}/list?pub_key={1}".format(entity, Exonum._public_key))
            except:
                pass
            else:
                if data.status_code == 200:
                    self.__processing(data.json())

    def __processing(self, data):
        for k, v in self._key_pair.items():
            for d in data:
                if d is not None and d.get(k) is not None:
                    keys = d.get(k).get(v[0]) + ":" + d.get(k).get(v[1])
                    if keys not in self.__processed_yet and sending(k, 'Новое поступление от: '+d.get(k).get(v[0])):
                        self.__processed_yet.append(d.get(k).get(v[0]) + ":" + d.get(k).get(v[1]))
