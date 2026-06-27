export function failAuthSurface({ prefix, message }) {
  console.error(`[${prefix}] ${message}`);
  process.exit(1);
}

export function parseAuthSurfaceLanguageArgs(
  argv,
  {
    prefix,
    supportedLanguages,
    defaultLanguages = supportedLanguages,
  },
) {
  const parsed = {
    languages: [],
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--language') {
      const value = (argv[index + 1] || '').trim().toLowerCase();
      if (!value) {
        failAuthSurface({ prefix, message: 'Missing value for --language' });
      }
      parsed.languages.push(value);
      index += 1;
      continue;
    }
    failAuthSurface({ prefix, message: `Unknown argument: ${current}` });
  }

  const languageSet = new Set(parsed.languages.length > 0 ? parsed.languages : defaultLanguages);
  for (const language of languageSet) {
    if (!supportedLanguages.includes(language)) {
      failAuthSurface({ prefix, message: `Unsupported language: ${language}` });
    }
  }

  return languageSet;
}

export function assertAbsent(failures, source, pattern, message) {
  if (pattern.test(source)) {
    failures.push(message);
  }
}

export function assertPresent(failures, source, pattern, message) {
  if (!pattern.test(source)) {
    failures.push(message);
  }
}

export function assertExactValues(failures, actualValues, expectedValues, message) {
  const actual = [...actualValues].sort();
  const expected = [...expectedValues].sort();
  if (actual.length !== expected.length || actual.some((value, index) => value !== expected[index])) {
    failures.push(
      `${message} Expected [${expected.join(', ')}] but found [${actual.join(', ')}].`,
    );
  }
}

export function finishAuthSurfaceVerification({
  prefix,
  failures,
  failureHeader = 'Auth surface alignment verification failed:',
  successMessage,
}) {
  if (failures.length > 0) {
    console.error(`[${prefix}] ${failureHeader}`);
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log(successMessage || `[${prefix}] Auth surface alignment verification passed.`);
}
