WITH RECURSIVE menu_tree (
path,
level,
parent_id,
id,
created_at,
updated_at,
removed_at,
name,
type
)
AS (
  SELECT
    array[]::TEXT[],
    0,
    parent_id,
    id,
    created_at,
    updated_at,
    removed_at,
    name,
    type
  FROM drive_files

  UNION ALL
  SELECT
    array_append(mt.path, CAST (fs.id AS TEXT)),
    mt.level + 1,
    mt.id,
    fs.id,
    fs.created_at,
    fs.updated_at,
    fs.removed_at,
    fs.name,
    fs.type
  FROM drive_files fs, menu_tree mt
  WHERE fs.parent_id = mt.id AND fs.removed_at IS NULL
)
SELECT * FROM menu_tree ORDER BY level;

598d2876-967d-48fa-b17f-9ec75f58c623


WITH ancestors AS (
WITH RECURSIVE tree AS (
  SELECT id, name, ARRAY[]::UUID[] AS ancestors
  FROM drive_files WHERE parent_id IS NULL

  UNION ALL

  SELECT drive_files.id, drive_files.name, tree.ancestors || drive_files.parent_id
  FROM drive_files, tree
  WHERE drive_files.parent_id = tree.id
) SELECT * FROM tree WHERE name = 'test3'
) SELECT drive_files.* FROM drive_files, ancestors WHERE drive_files.id = ANY(ancestors.ancestors);


WITH ancestors AS (
	WITH RECURSIVE tree AS (
	  SELECT id, name, ARRAY[]::UUID[] AS ancestors
	  FROM drive_files WHERE parent_id IS NULL

	  UNION ALL

	  SELECT drive_files.id, drive_files.name, tree.ancestors || drive_files.parent_id
	  FROM drive_files, tree
	  WHERE drive_files.parent_id = tree.id
	) SELECT * FROM tree WHERE id = '1e29fa22-a0c4-4315-9364-411608851e01'
) SELECT drive_files.id, drive_files.name FROM drive_files, ancestors WHERE drive_files.id = ANY(ancestors.ancestors);
