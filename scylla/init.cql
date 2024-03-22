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
    order_id  bigint,
    direction tinyint,
    currency  text,
    initiator bigint,
    associate bigint,
    amount    text,
    remark    text,
    create_at bigint,
    -- 基于关联用户查询
    PRIMARY KEY (associate, currency, direction, create_at, order_id)
) WITH CLUSTERING ORDER BY (currency ASC, direction ASC, create_at DESC);

-- 基于订单Id查询
CREATE INDEX IF NOT EXISTS ON orders (order_id);
-- 基于发起者查询
CREATE INDEX IF NOT EXISTS ON orders (initiator);