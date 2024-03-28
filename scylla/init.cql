CREATE KEYSPACE IF NOT EXISTS biz
    WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor' : 1};

use biz;

-- CREATE TYPE IF NOT EXISTS u128
-- (
--     high bigint,
--     low  bigint,
-- );

CREATE TABLE IF NOT EXISTS orders
(
    oid bigint,
    txn tinyint,
    ccy text,
    mch bigint,
    usr  bigint,
    amt text,
    rmk text,
    ctm bigint,
    -- 基于关联用户查询
    PRIMARY KEY (usr, ccy, txn, ctm, oid)
) WITH CLUSTERING ORDER BY (ccy ASC, txn ASC, ctm DESC);

-- 基于订单Id查询
CREATE INDEX IF NOT EXISTS ON orders (oid);
-- 基于发起者查询
CREATE INDEX IF NOT EXISTS ON orders (mch);