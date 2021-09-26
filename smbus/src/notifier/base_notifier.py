# -*- coding: utf-8 -*-
"""
Descrition
"""
__author__ = 'maxdob'
__date__ = '25.09.21'
__time__ = '21:32'
__version__ = '0.1'


class BaseNotifier:
    """BaseNotifier class provides basic functionality."""

    def __init__(self, message: str):
        """
        Constructor of Notifier.

        Parameters
        ----------
        message: str
            Message to send
        """
        self._message = message

    def _create_message(self) -> str:
        return self._message

    def send(self) -> bool:
        raise NotImplementedError("Please Implement this method")
