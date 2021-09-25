# -*- coding: utf-8 -*-
"""
Descrition
"""
__author__ = 'maxdob'
__date__ = '25.09.21'
__time__ = '17:07'
__version__ = '0.1'

import pika
import settings


def on_message(channel, method_frame, prop, body):
    body_str = body.decode("utf-8")
    rkey = method_frame.routing_key
    print(body_str)
    print(rkey, method_frame.delivery_tag)
    print(channel.queue, prop)
    channel.basic_ack(delivery_tag=method_frame.delivery_tag)


rmq_parameters = pika.URLParameters(settings.rmq_url_connection_str)
rmq_connection = pika.BlockingConnection(rmq_parameters)
rmq_channel = rmq_connection.channel()
rmq_channel.queue_declare(queue='inserer.one', durable = True)
rmq_channel.basic_consume(on_message_callback = on_message, queue='inserer.one', auto_ack=True)

try:
    rmq_channel.start_consuming()
# except KeyboardInterrupt:
#     rmq_channel.stop_consuming()
except Exception:
    rmq_channel.stop_consuming()