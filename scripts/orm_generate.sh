
# 定义变量
DB_URL="postgres://postgres:postgres@localhost:5432/mail-server"
OUTPUT_PATH="src/model"
MIGRATION_ACTION="up"  # 默认为 up



# 执行迁移
echo "执行迁移: $MIGRATION_ACTION"
sea-orm-cli migrate $MIGRATION_ACTION


  echo "生成实体"
  sea-orm-cli generate entity \
    -u "$DB_URL" \
    -o "$OUTPUT_PATH" \
    -v \
    --with-serde both

echo "脚本执行完成"
