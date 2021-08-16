from libs.libsai_python import Derivation, Sai

sai = Sai.derive(Derivation.Blake3_256, "something to hash")
print(sai)
assert Sai.verify(sai, "something to hash")
assert not Sai.verify(sai, "something else")