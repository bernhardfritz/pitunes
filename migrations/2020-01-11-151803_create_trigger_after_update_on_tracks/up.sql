CREATE TRIGGER after_update_on_tracks
AFTER UPDATE ON tracks
FOR EACH ROW
BEGIN
    UPDATE tracks SET updated_at = CURRENT_TIMESTAMP
    WHERE id = old.id;
END