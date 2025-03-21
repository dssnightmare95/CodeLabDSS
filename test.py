import requests
import time
from datetime import datetime, timedelta

# Configurar tu API Key de Etherscan
ETHERSCAN_API_KEY = "KDI4WVYA3X1324IXPSQIXZBKYJZYI3RUZ7"

# Tiempo actual y hace una hora (en timestamp UNIX)
end_time = int(time.time())
start_time = end_time - 3600*24*7  # Una hora atrás

# Endpoint para obtener contratos creados
url = f"https://api.etherscan.io/api?module=account&action=txlist&address=0x0000000000000000000000000000000000000000&startblock=0&endblock=99999999&sort=desc&apikey={ETHERSCAN_API_KEY}"

response = requests.get(url)
data = response.json()

if data["status"] == "1":  # Revisión de respuesta exitosa
    new_tokens = []

    for tx in data["result"]:
        if start_time <= int(tx["timeStamp"]) <= end_time:
            contract_address = tx["contractAddress"]

            # Consultar detalles del contrato
            token_info_url = f"https://api.etherscan.io/api?module=contract&action=getsourcecode&address={contract_address}&apikey={ETHERSCAN_API_KEY}"
            token_response = requests.get(token_info_url).json()

            if token_response["status"] == "1" and token_response["result"]:
                contract_info = token_response["result"][0]
                token_name = contract_info.get("ContractName", "Desconocido")
                token_symbol = contract_info.get("TokenSymbol", "N/A")

                new_tokens.append({
                    "name": token_name,
                    "symbol": token_symbol,
                    "contract": contract_address
                })

    if new_tokens:
        print("\n📌 Tokens creados en la última hora:")
        for token in new_tokens:
            print(f"🔹 {token['name']} ({token['symbol']}) - {token['contract']}")
    else:
        print("No se encontraron nuevos tokens en la última hora.")

else:
    print("Error al obtener datos:", data["message"])
