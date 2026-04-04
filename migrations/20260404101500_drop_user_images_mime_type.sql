ALTER TABLE user_images
DROP CONSTRAINT IF EXISTS user_images_source_ck,
DROP COLUMN IF EXISTS mime_type,
ADD CONSTRAINT user_images_source_ck
    CHECK (num_nonnulls("url", bytes) = 1);
