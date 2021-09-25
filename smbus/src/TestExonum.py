"""Example of a Basic API Interaction via Exonum Python Light Client."""
from exonum_client import ExonumClient
from exonum_client.client import Subscriber
from exonum_client.crypto import KeyPair, PublicKey, SecretKey
from pysodium import (
    crypto_sign_keypair,
crypto_sign_seed_keypair,
crypto_sign_SEEDBYTES
)

public_key = b'vl\x96&\x82+_\xfdL\xcd(\xbb\xa7\xd1\x19H\x91\x88\x8c\xbf\xb8\xbd\x13d?\xeb\x1c8\x0b\xd9J\xfd'
secret_key = b'5154181e45fgefq45w4fgwe98df9s28wvl\x96&\x82+_\xfdL\xcd(\xbb\xa7\xd1\x19H\x91\x88\x8c\xbf\xb8\xbd\x13d?\xeb\x1c8\x0b\xd9J\xfd'

pub_ui = "7438d788a2dca76abbbb9051780c73159bd811b908ff3b6fcd8a4cc4d3234706"

def key_run() -> None:
    """This example creates two wallets (for Alice and Bob) and performs several
    transactions between these wallets."""
    # client = ExonumClient(hostname="127.0.0.1", public_api_port=7777, private_api_port=8888)

    # public_key,secret_key = crypto_sign_seed_keypair(b"5154181e45fgefq45w4fgwe98df9s28w")
    # key_pair = KeyPair(PublicKey(public_key), SecretKey(secret_key))
    # print(crypto_sign_SEEDBYTES)
    print(public_key)
    print(secret_key)


def sub_run()-> None:
    # sub = Subscriber(address="185.174.235.90", port=7777, subscription_type="transactions")
    # sub.run()
    client = ExonumClient(hostname="185.174.235.90", public_api_port=7777, private_api_port=8888)
    with client.create_subscriber(subscription_type="transactions") as subscriber:
        subscriber.wait_for_new_event()
        subscriber.wait_for_new_event()

    # with client.create_subscriber("blocks") as subscriber:
    #     subscriber.wait_for_new_event()
    #     subscriber.wait_for_new_event()

def runrun() -> None:
    """Example of a simple API interaction."""
    client = ExonumClient(hostname="185.174.235.90", public_api_port=7777, private_api_port=8888)
    tx_info = client.public_api.get_tx_info('72b3e197ae6349e0da8523372db4b4000a97130796efa43f6edb233c547365d1')
    # print(tx_info.json())
    blocks = client.public_api.get_blocks(5)
    # print(blocks.json())
    pinfo = client.private_api.get_info()
    print(pinfo.json())
    block = client.public_api.get_block(17858)
    # print(block.json())
    service_public_api = client.service_public_api("crypto")
    pub_key = PublicKey(public_key).__str__()

    list_loan = service_public_api.get_service("v1/loan_request/list?pub_key=" + pub_key).json()
    print(list_loan)


def run() -> None:
    """Example of a simple API interaction."""
    client = ExonumClient(hostname="185.174.235.90", public_api_port=7777, private_api_port=8888)
    # Get the available services:
    print("Available services:")

    available_services_response = client.public_api.available_services()

    if available_services_response.status_code == 200:
        available_services = available_services_response.json()
        print(" Artifacts:")
        for artifact in available_services["artifacts"]:
            print(f"  - {artifact['name']}:{artifact['version']} (runtime ID {artifact['runtime_id']})")
            # service_public_api = client.service_public_api(artifact['name'])
            # print(service_public_api)
        print(" Instances:")
        for state in available_services["services"]:
            instance = state["spec"]
            print(f"  - ID {instance['id']} => {instance['name']} (artifact {instance['artifact']['name']})")
    else:
        print("Available services request failed")
    print("")


    # Get the health info:
    print("Node info:")
    node_info_response = client.private_api.get_info()
    if node_info_response.status_code == 200:
        node_info = node_info_response.json()
        print(f"Consensus status: {node_info['consensus_status']}")
        print(f"Connected peers: {node_info['connected_peers']}")
    else:
        print("Node info request failed.")
    print("")

    # Get the Exonum stats:
    print("Exonum stats:")
    stats_response = client.private_api.get_stats()
    if stats_response.status_code == 200:
        stats = stats_response.json()
        print(f"Current height: {stats['height']}")
        print(f"Tx pool size: {stats['tx_pool_size']}")
        print(f"Tx count: {stats['tx_count']}")
        print(f"Tx cache size: {stats['tx_cache_size']}")
    else:
        print("Stats request failed.")
    print("")

    # with client.create_subscriber(subscription_type="transactions") as subscriber:
    #     subscriber.wait_for_new_event()
    #     subscriber.wait_for_new_event()


if __name__ == "__main__":
    # import ctypes
    # import ctypes.util
    #
    # sodium = ctypes.cdll.LoadLibrary(ctypes.util.find_library('sodium') or ctypes.util.find_library('libsodium'))
    # if not sodium._name:
    #     raise ValueError('Unable to find libsodium')

    # service_public_api = client.service_public_api(instance_name)
    # alice_wallet = service_public_api.get_service("v1/wallets/info?pub_key=" + alice_keys.public_key.hex()).json()
    runrun()
    # sub_run()
    # key_run()