CREATE KEYSPACE IF NOT EXISTS biz
    WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor' : 1};

use biz;

CREATE TABLE IF NOT EXISTS orders
(
    oid  bigint,
    side text,
    ccy  text,
    mch  bigint,
    usr  bigint,
    amt  text,
    rmk  text,
    ctm  bigint,
    PRIMARY KEY (usr, ctm, ccy, side, mch)
);
-- select * from orders where usr = 0 and ctm > 0 and ctm < 100 and ccy in ('a') and side in ('a', 'b') allow filtering;

-- 基于订单查询
CREATE INDEX IF NOT EXISTS ON orders (oid);
-- select * from orders where oid = 0;

-- 商户视角查询
CREATE MATERIALIZED VIEW orders_by_mch AS
SELECT * FROM orders
WHERE mch IS NOT NULL
  AND ccy IS NOT NULL
  AND side IS NOT NULL
  AND ctm IS NOT NULL
  AND usr IS NOT NULL
PRIMARY KEY (mch, ctm, ccy, side, usr);
-- select * from orders where usr = 0 and ctm > 0 and ctm < 100 and ccy in ('a') and side in ('a', 'b') allow filtering;