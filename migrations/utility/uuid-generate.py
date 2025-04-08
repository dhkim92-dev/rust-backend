import time
import random
import uuid
from uuid import UUID
from datetime import datetime


class UUIDV7:
    def __init__(self):
        timestamp = int(time.time() * 1000)
        timestamp &= 0x0000FFFFFFFFFFFF

        random_bits = random.getrandbits(12)
        most_sig_bits = (timestamp << 16) | 0x7000 | random_bits
        random_bits = random.getrandbits(62)
        least_sig_bits = (0b10 << 62) | random_bits
        self.uuid = uuid.UUID(int=(most_sig_bits << 64) | least_sig_bits)

    @staticmethod
    def from_timestamp(timestamp: int): 
        timestamp &= 0x0000FFFFFFFFFFFF
        random_bits = random.getrandbits(12)
        most_sig_bits = (timestamp << 16) | 0x7000 | random_bits
        random_bits = random.getrandbits(62)
        least_sig_bits = (0b10 << 62) | random_bits
        return uuid.UUID(int=(most_sig_bits << 64) | least_sig_bits)

    def to_string(self):
        return str(self.uuid)


class SQLGenerator:
    def __init__(self):
        self.data = []

    def register(self, key: UUID, value: datetime):
        self.data.append((key, value))

    def cvt_to_uuid_v7(self, ts: int):
        return UUIDV7.from_timestamp(ts)

    def generate_sql(self): 
        sql_statements = [] 
        # """
        for key, value in self.data:
            sql_statements.append(f"UPDATE article SET id = \
                    '{UUIDV7.from_timestamp(value)}' WHERE  = '{key}';\n")

        return sql_statements

    def print_sql(self):
        sql_statements = self.generate_sql()
        for statement in sql_statements:
            print(statement)

def from_file(fname): 
    with open(fname, 'r') as f :
        lines = f.readlines()

        for line in lines:
            key, value = line.strip().split(',')
            key = UUID(key)
            value = datetime.fromisoformat(value)

if __name__ == '__main__' :
    for _ in range(0, 10) :
        print(UUIDV7().to_string())
