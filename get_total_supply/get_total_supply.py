import os
import requests

API_KEY = "KDI4WVYA3X1324IXPSQIXZBKYJZYI3RUZ7"
WALLET = '0xcc88F94D0F50aD502a83db88CF5637aE1391184D'
Wei2ETH = 1000000000000000000

def get_ether_balance_for_single_address(Address, ApiKeyToken):
    url = f'https://api.etherscan.io/api?module=account&action=balance&address={Address}&tag=latest&apikey={ApiKeyToken}'
    response = requests.get(url)
    data = response.json()
    if data['status'] == '0':
       return 'Error: ' + data['message'] 
    else:
        return 'Balance for ' + Address + ' is ' + str(int(data['result']) / Wei2ETH) + ' ETH'


print(get_ether_balance_for_single_address(WALLET, API_KEY))

