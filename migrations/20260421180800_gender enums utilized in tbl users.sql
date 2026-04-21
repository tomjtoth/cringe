ALTER TABLE users 
ADD COLUMN gender_identity gender_identity,
ADD CONSTRAINT gender_identity_ck CHECK (
    gender = 'male' AND (
        gender_identity IS NULL
        OR gender_identity in (
            'demimale',
            'gender fluid',
            'gender questioning',
            'genderqueer',
            'intersex man',
            'trans man',
            'transmasculine'
        )
    )
    OR gender = 'female' AND (
        gender_identity IS NULL
        OR gender_identity in (
            'demifemale',
            'gender fluid',
            'gender questioning',
            'genderqueer',
            'intersex woman',
            'trans woman',
            'transfeminine'
        )
    )
    OR gender = 'non-binary' AND (
        gender_identity IS NULL
        OR gender_identity NOT IN (
            'demimale',
            'demifemale'
        )
    )
);
