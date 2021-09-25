# -*- coding: utf-8 -*-

import pika
from pika.exchange_type import ExchangeType
import settings

def on_message(channel, method_frame, prop, body):
    body_str = body.decode("utf-8")
    rkey = method_frame.routing_key
    print(body_str)
    print(rkey)
    channel.basic_ack(delivery_tag=method_frame.delivery_tag)


rmq_parameters = pika.URLParameters(settings.rmq_url_connection_str)
rmq_connection = pika.BlockingConnection(rmq_parameters)
rmq_channel = rmq_connection.channel()
rmq_channel.queue_declare(queue='test', durable = True)
rmq_channel.exchange_declare(exchange='vtb_154af541', exchange_type=ExchangeType.topic.value, durable=True)
rmq_channel.basic_publish(exchange='vtb_154af541', routing_key="test", body=b"testhellohellohellohellohellohello") #properties


rmq_channel.basic_consume(on_message_callback = on_message, queue='test')
try:
    rmq_channel.start_consuming()
# except KeyboardInterrupt:
    rmq_channel.stop_consuming()
except Exception:
    rmq_channel.stop_consuming()
    # rmq_channel.console_log("Ошибка:\n", traceback.format_exc())



rmq_connection.close()
