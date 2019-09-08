WITH RECURSIVE tree AS (
  SELECT id, size, type, ARRAY[]::UUID[] AS ancestors
  FROM drive_files WHERE parent_id IS NULL

  UNION ALL

  SELECT drive_files.id, drive_files.size, drive_files.type, tree.ancestors || drive_files.parent_id
  FROM drive_files, tree
  WHERE drive_files.parent_id = tree.id
) SELECT id, size, type AS type_ FROM tree WHERE $1 = ANY(tree.ancestors);
