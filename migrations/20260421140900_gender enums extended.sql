ALTER TYPE gender ADD VALUE 'non-binary' AFTER 'female';

CREATE TYPE gender_identity AS ENUM (
    'agender',
    'bigender',
    'demimale',
    'demifemale',
    'gender fluid',
    'gender nonconforming',
    'gender questioning',
    'gender variant',
    'genderqueer',
    'intersex',
    'intersex man',
    'intersex woman',
    'neutrosis',
    'pangender',
    'polygender',
    'trans man',
    'trans woman',
    'transfeminine',
    'transgender',
    'transmasculine',
    'two-spirit'
);
