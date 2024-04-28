import { initializeFirebase, auth } from '$lib/firebase.client';
import { browser } from '$app/environment';
import { onAuthStateChanged, type User } from '@firebase/auth';
import type { LayoutLoad } from './$types';

export const load = (async ({ url }) => {
	if (browser) {
		try {
			initializeFirebase();
		} catch (ex) {
			console.error(ex);
		}
	}

	function getAuthUser(): Promise<User | null> {
		return new Promise((resolve) => {
			onAuthStateChanged(auth, (user) => resolve(user));
		});
	}

	return {
		getAuthUser: getAuthUser,
		url: url.pathname
	};
}) satisfies LayoutLoad;
