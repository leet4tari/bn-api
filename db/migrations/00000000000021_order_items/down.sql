DROP INDEX IF EXISTS index_order_items_order_id;
DROP INDEX IF EXISTS index_order_items_ticket_type_id;
DROP INDEX if EXISTS INDEX index_order_items_event_id;
ALTER TABLE order_items DROP CONSTRAINT constraint_order_items_ticket_type_id;
DROP TABLE IF EXISTS order_items;
