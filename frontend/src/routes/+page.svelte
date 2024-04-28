<script lang="ts">
	// login/+page.svelte
	import { session } from '$lib/session';
	import { auth } from '$lib/firebase.client';
	import {
		createUserWithEmailAndPassword,
		signInWithEmailAndPassword,
		type UserCredential
	} from '@firebase/auth';
	import { goto } from '$app/navigation';

	let loginEmail: string = '';
	let loginPassword: string = '';

	let registerationEmail: string = '';
	let registerationPassword: string = '';

	async function loginWithMail() {
		await signInWithEmailAndPassword(auth, loginEmail, loginPassword)
			.then((result) => {
				const { user }: UserCredential = result;
				session.set({
					loggedIn: true,
					user
				});
				goto('/protected');
			})
			.catch((error) => {
				return error;
			});
	}

	async function handleRegister() {
		await createUserWithEmailAndPassword(auth, registerationEmail, registerationPassword)
			.then((result) => {
				const { user } = result;
				session.update((cur) => {
					return {
						...cur,
						user,
						loggedIn: true,
						loading: false
					};
				});
				goto('/protected');
			})
			.catch((error) => {
				throw new Error(error);
			});
	}
</script>

<div class="login-form">
	<h1>Login</h1>
	<form on:submit={loginWithMail}>
		<input bind:value={loginEmail} type="text" placeholder="Email" />
		<input bind:value={loginPassword} type="password" placeholder="Password" />
		<button type="submit">Login</button>
	</form>
</div>

<div class="register-form">
	<form on:submit={handleRegister}>
		<h2>Register</h2>
		<input bind:value={registerationEmail} type="text" placeholder="Email" />
		<input bind:value={registerationPassword} type="password" placeholder="Password" />
		<button type="submit">Register</button>
	</form>
</div>
