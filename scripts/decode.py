from stellar_base.stellarxdr import Xdr
import base64
import jsonpickle
import xdrlib

print("Hello world")

data = 'AAAAAAAB8ywAAAAAAAAAAQAAAAAAAAAYAAAAAIguJa302fEsLry803Mc4PRzklemc+hgwC+m7utJ7FyHAAAAAA=='

result_bytes = base64.b64decode(data)
# tx_result = Xdr.StellarXDRUnpacker(result_bytes).unpack_TransactionResult()

# p = jsonpickle.Pickler(keys=True)
# print(p.flatten(tx_result))

print(repr(result_bytes))

u = xdrlib.Unpacker(result_bytes)

# print(u.unpack_uint())
print(u.unpack_array(result_bytes))

# u.done()