window.__TAURI_ISOLATION_HOOK__ = (payload) => {
    // let's validate the regex pattern here to make sure it's present and a valid pattern
	if (payload.cmd === 'search') {
		if (!('pattern' in payload)) return new Error('expected pattern key in payload');

		try {
			new RegExp(payload.pattern);
			return null;
		} catch (e) {
			return null;
		}
	}

	return payload;
};
