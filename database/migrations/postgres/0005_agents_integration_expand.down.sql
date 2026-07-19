BEGIN;

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM im_projection_conversation_agent LIMIT 1)
       OR EXISTS (SELECT 1 FROM im_conversation_agent_binding LIMIT 1)
       OR EXISTS (SELECT 1 FROM im_agent_dispatch LIMIT 1) THEN
        RAISE EXCEPTION
            'IM Agents 2.0 rollback refused: integration tables contain durable data';
    END IF;
END;
$$;

DROP TABLE im_agent_dispatch;
DROP TABLE im_conversation_agent_binding;
DROP TABLE im_projection_conversation_agent;

COMMIT;
