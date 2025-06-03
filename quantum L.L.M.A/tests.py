# tests/test_encryption.py

import os
import sys
import unittest

# Add the path to the encryption module that resides in
# "quantum A.I. General Agent system prototype/encryption.py"
MODULE_DIR = os.path.join(os.path.dirname(__file__), '..', 'quantum A.I. General Agent system prototype')
if MODULE_DIR not in sys.path:
    sys.path.insert(0, MODULE_DIR)

from encryption import EncryptionManager

class TestEncryptionManager(unittest.TestCase):
    def test_encryption_decryption(self):
        manager = EncryptionManager()
        plaintext = "Test message"
        encrypted = manager.encrypt(plaintext)
        # Ensure that the encrypted data differs from the original plaintext
        self.assertNotEqual(encrypted, plaintext)
        decrypted = manager.decrypt(encrypted)
        self.assertEqual(plaintext, decrypted)

if __name__ == '__main__':
    unittest.main()
