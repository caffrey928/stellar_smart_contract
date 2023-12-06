import requests
import json
import base64

TRUVITY_API_KEY="PIdRyS2oiRxrp-M0NTFqAiTRBaRgcokurBUpnewgAcgJL6wAAf0rF-i95p3bFP-BfLOC_lECXTPpUvlW7-ytLR71y-088Lkcinsxa9tcL-PdbCxuzHsgPBpeJ7Q2zjYfOUT6nsxy3dWwuJmKwPap86MT20ZWqffpQJm4KpoMn-CwOVPMlVrTCbbyM8Ncf-r1fZFp_We796lAg92z-nAHXTj-FYUMM4-H2TxZ3WcCFZQcVzL9dQ_zVLmxCKr1BxoOoCBFa7oG3IX1z4d69FW7L8ESbEBmGET_tPNBNeipUl4khYg_FnPacrgQSAQY8FT9CNSU1MyctNL8IHG9D_TdJQ"
url="https://api.truvity.com/api/wallet/v1/entity"
header = {
                "X-API-KEY" : TRUVITY_API_KEY,
                "Content-Type" : "application/json"
          
}
# res = requests.post(url, headers=header)
# print(res.text)


ENTITY_DID="did:key:zDnaeR7tAY6mKiebWGTnKNzkJ5TxEk6Wq7FfiVPnxSSPs1ZVU"
url="https://api.truvity.com/api/wallet/v1/wallet?entityDid={}".format(ENTITY_DID)
#res = requests.post(url, headers=header)
#print(res.text)

WALLET_DID="did:key:zDnaed5j9Gn8ZZzFibbiBWERSa7cUXRmr6yj3GqrZBZW64wwy"
TARGET_ENTITY_DID="did:key:zDnaezDUJ3JUbLDcJJ7BaX7f2kcx5y7TkWGUsHHmEjWLCuUaZ"
TARGET_WALLET_DID="did:key:zDnaeyZMQfi5wh1ByhJuAHeccU3Z7idvoV9L8bH9WsmJUUR6A"

url="https://api.truvity.com/api/wallet/v1/credential?entityDid={}&walletDid={}".format(ENTITY_DID, WALLET_DID)
# res = requests.post(url, headers=header)
# print(res.text)

CREDENTIAL_DID="did:uuid:33ff94d7-a1d7-4562-9cb6-91d48a9ea2fe"
with open("credential.json") as file:
    Credential = json.load(file)
Credential=json.dumps(Credential)
Credential=base64.b64encode(Credential.encode())
#print(Credential.decode('utf-8'))
url="https://api.truvity.com/api/wallet/v1/credential/{}?revision={}&entityDid={}&walletDid={}".format(CREDENTIAL_DID, 1, ENTITY_DID, WALLET_DID)
payload = "{\"JsonLd\" : \"%s\"}" % Credential.decode('utf-8')
# res = requests.patch(url, headers=header, data=payload)
# print(res.text)

url=f"https://api.truvity.com/api/wallet/v1/credential/{CREDENTIAL_DID}?revision=2&entityDid={ENTITY_DID}&walletDid={WALLET_DID}"
res = requests.post(url, headers=header)
print(res.text)

# VC Verification
url=f"https://api.truvity.com/api/wallet/v1/credential/{CREDENTIAL_DID}?walletDid={WALLET_DID}&entityDid={ENTITY_DID}"
# res = requests.get(url, headers=header)
# Credential_Ld=json.loads(res.text)["JsonLd"]
# url="https://api.truvity.com/api/wallet/v1/credentials/verification"
# payload = "{\"JsonLd\" : \"%s\"}" % Credential_Ld
# res = requests.post(url, headers=header, data=payload)
# print(res)

url=f"https://api.truvity.com/api/wallet/v1/vp?entityDid={ENTITY_DID}&walletDid={WALLET_DID}"
# res = requests.post(url, headers=header)
# print(res.text)

PRESENTATION_DID="did:uuid:f0654ece-407b-4d12-bd53-1fe956c801a8"
url=f"https://api.truvity.com/api/wallet/v1/vp/from-vc/{PRESENTATION_DID}?revision=3&entityDid={ENTITY_DID}&walletDid={WALLET_DID}"
payload = "{\"CredentialDid\" : \"%s\"}" % CREDENTIAL_DID
# res = requests.post(url, headers=header, data=payload)
# print(res.text)

url=f"https://api.truvity.com/api/wallet/v1/vp/{PRESENTATION_DID}?revision=3&entityDid={ENTITY_DID}&walletDid={WALLET_DID}"
# res = requests.post(url, headers=header)
# print(res.text)

# DIDComm is not ready yet
# TARGET_ENTITY_DID="did:key:zDnaezDUJ3JUbLDcJJ7BaX7f2kcx5y7TkWGUsHHmEjWLCuUaZ"
# TARGET_WALLET_DID="did:key:zDnaeyZMQfi5wh1ByhJuAHeccU3Z7idvoV9L8bH9WsmJUUR6A"
# url=f"https://api.truvity.com/api/wallet/v1/didcomm/send-plaintext?entityDid={ENTITY_DID}&walletDid={WALLET_DID}"
# payload = "{\"Target\" : {\"EntityDid\" : \"%s\", \"WalletDid\" : \"%s\"}, \"VpDid\": \"%s\"}" % (TARGET_ENTITY_DID, TARGET_WALLET_DID, PRESENTATION_DID)
# res = requests.post(url, headers=header, data=payload)
# print(res)

# VP Verification
url=f"https://api.truvity.com/api/wallet/v1/vp/{PRESENTATION_DID}?walletDid={WALLET_DID}&entityDid={ENTITY_DID}"
# res = requests.get(url, headers=header)
# Presentation_Ld=json.loads(res.text)["JsonLd"]
# url="https://api.truvity.com/api/wallet/v1/vp/verification"
# payload = "{\"JsonLd\" : \"%s\"}" % Presentation_Ld
# res = requests.post(url, headers=header, data=payload)
# print(res)