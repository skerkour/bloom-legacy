package repository

/*
queryInsert := `INSERT INTO objects
					(id, updated_at_state, algorithm, nonce, encrypted_key, encrypted_data, group_id)
					VALUES ($1, $2, $3, $4, $5, $6, $7)
				`
				_, err = tx.Exec(queryInsert, object.ID, object.UpdatedAtState, object.Algorithm,
					object.Nonce, object.EncryptedKey, object.EncryptedData, object.GroupID)
				if err != nil {
					logger.Error("sync.push: inserting object",
						rz.String("object.id", base64.StdEncoding.EncodeToString(object.ID)),
						rz.String("object.group_id", object.GroupID.String()),
					)
					return
				}
*/
