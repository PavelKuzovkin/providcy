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

    def __init__(self, subject: str, message: str):
        """
        Constructor of Notifier.

        Parameters
        ----------
        subject: str
            Examples: 'Hi'.
        message: str
            Message to send
        """
        self._subject = subject
        self._message = message

    def send(self) -> bool:
        raise NotImplementedError("Please Implement this method")
