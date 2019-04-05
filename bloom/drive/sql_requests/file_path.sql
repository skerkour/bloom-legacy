WITH ancestors AS (
    WITH RECURSIVE tree AS (
        SELECT id, name, ARRAY[]::UUID[] AS ancestors
        FROM drive_files WHERE parent_id IS NULL

        UNION ALL

        SELECT drive_files.id, drive_files.name, tree.ancestors || drive_files.parent_id
        FROM drive_files, tree
        WHERE drive_files.parent_id = tree.id
    ) SELECT * FROM tree WHERE id = $1
) SELECT drive_files.id, drive_files.name FROM drive_files, ancestors WHERE drive_files.id = ANY(ancestors.ancestors) OR drive_files.id = $1;
