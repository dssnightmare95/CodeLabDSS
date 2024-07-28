import os
import requests
import json

API_KEY = "HP492B8BVFRCANT24SZHIZKBUACCYNF2SQ"
CONTRACT_ADDRESS = "0xadbf2fc548ba31941477d23c15697e7eece1b5f5"
Wei2ETH = 1000000000000000000

def get_contract_all_tx_list(ContracAddress, ApiKeyToken):
    url = f'https://api.bscscan.com/api?module=account&action=txlist&address={ContracAddress}&apikey={ApiKeyToken}'
    response = requests.get(url)
    data = response.json()
    if data['status'] == '0':
        print('get_contract_all_tx_list Error: ', data['message'])
        return []
    else:
        return data['result']

def get_ether_balance_for_multiple_addr_in_single_call(Wallets, ApiKeyToken):
    url = f'https://api.bscscan.com/api?module=account&action=balancemulti&address={Wallets}&tag=latest&apikey={ApiKeyToken}'
    response = requests.get(url)
    data = response.json()
    if data['status'] == '0':
        print('get_ether_balance_for_multiple_addr_in_single_call Error: ', data['message'])
        return []
    else:
        return data['result']

def get_all_wallets_in_contract_balance(ContracAddress, ApiKeyToken):
    all_contract_transactions = get_contract_all_tx_list(ContracAddress, ApiKeyToken)
    unique_wallets = []
    wallets_balances = {}
    all_list_wallets = ''
    aux_wallet_count = 0
    for transaction in all_contract_transactions:
        if transaction['to'] not in unique_wallets:
            unique_wallets.append(transaction['to'])
        if transaction['from'] not in unique_wallets:
            unique_wallets.append(transaction['from'])
    for wallet in unique_wallets:
        if wallet != '' and aux_wallet_count < 20:
            all_list_wallets = all_list_wallets + "," + wallet
            aux_wallet_count += 1
        if aux_wallet_count == 20:
            for elements in get_ether_balance_for_multiple_addr_in_single_call(all_list_wallets[1:], ApiKeyToken):
                wallets_balances[elements['account']] = int(elements['balance']) / Wei2ETH
            aux_wallet_count = 0
            all_list_wallets = ''
    return wallets_balances

#### RESULT ####
results = get_all_wallets_in_contract_balance(CONTRACT_ADDRESS, API_KEY)
print('The wallets and their balances in the contract ', CONTRACT_ADDRESS, ' are:' )
print(type(results))
for wallet, balance in results.items():
    print('Wallet:: ', wallet, " have ", balance, " BNB")

archive_name = 'wallets.json'
with open(archive_name, 'w') as archive:
    json.dump(results, archive, indent=4)