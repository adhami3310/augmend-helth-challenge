<script lang="ts">
	import { session } from '$lib/session';

	// Form Fields
	let name = '';
	let email = $session.user?.email;
	let age: number | undefined = undefined;

	let maritalStatus: 'single' | 'married' | 'widowed' | 'other' = 'single';
	let otherMaritalStatus = '';

	let seenTherapist = false;

	let takenMeds = false;
	let meds = [''];

	let statusMessage = '';

	// Indicates if the user has submitted the form before
	let submittedBefore = fetch('http://127.0.0.1:8080/survey/' + $session.user?.uid, {
		mode: 'cors'
	})
		.then((response) => response.json())
		.then((status) => status == true);

	async function onSubmit() {
		const fixedMaritalStatus =
			maritalStatus && maritalStatus === 'other' ? { other: otherMaritalStatus } : maritalStatus;

		const response = await fetch('http://127.0.0.1:8080/survey/' + $session.user?.uid, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			mode: 'cors',
			body: JSON.stringify({
				name,
				email,
				age,
				maritalStatus: fixedMaritalStatus,
				seenTherapist,
				meds: takenMeds ? meds : []
			})
		});

		if (response.ok) {
			statusMessage = 'Form Submitted Successfully';
		} else {
			statusMessage = 'Something Went Wrong';
		}
	}
</script>

{#if statusMessage}
	{statusMessage}
{:else}
	{#await submittedBefore}
		<div>Loading...</div>
	{:then submittedBefore}
		{#if submittedBefore}
			<div>Form Submitted Already</div>
		{:else}
			<form on:submit|preventDefault={onSubmit}>
				<div>
					<label for="name">Name</label>
					<input
						id="name"
						name="name"
						type="text"
						bind:value={name}
						placeholder="John Doe"
						required
					/>
				</div>

				<div>
					<label for="email">Email</label>
					<input
						id="email"
						name="email"
						type="email"
						bind:value={email}
						placeholder="Email"
						required
					/>
				</div>

				<div>
					<label for="age">What is your age?</label>
					<input id="age" name="age" type="number" bind:value={age} min="1" max="125" required />
				</div>

				<fieldset role="radiogroup">
					<legend>Which of the following best describes your marital status?</legend>
					<label
						><input
							id="marital-status-single"
							required
							type="radio"
							bind:group={maritalStatus}
							value="single"
						/>Single</label
					>
					<label
						><input
							id="marital-status-married"
							required
							type="radio"
							bind:group={maritalStatus}
							value="married"
						/>Married</label
					>
					<label
						><input
							id="marital-status-widowed"
							required
							type="radio"
							bind:group={maritalStatus}
							value="widowed"
						/>Widowed</label
					>
					<label
						><input
							id="marital-status-other"
							required
							type="radio"
							value="other"
							bind:group={maritalStatus}
						/>Other</label
					>
					{#if maritalStatus && maritalStatus === 'other'}
						<input
							type="text"
							name="marital-status-text"
							id="marital-status-text"
							placeholder="Marital Status"
							required
							bind:value={otherMaritalStatus}
						/>
					{/if}
				</fieldset>

				<div>
					<label>
						<input
							type="checkbox"
							name="seen-therapist"
							id="seen-therapist"
							bind:checked={seenTherapist}
						/>
						I have seen a therapist for mental health issues
					</label>
				</div>

				<div>
					<label>
						<input type="checkbox" name="taken-meds" id="taken-meds" bind:checked={takenMeds} />
						I am taking medications
					</label>
					{#if takenMeds}
						{#each meds as med, index}
							<div class="meds-line">
								<input
									type="text"
									name="meds-{index}"
									id="meds-{index}"
									placeholder="Medication Name"
									bind:value={med}
									required
								/>
								<button
									type="button"
									on:click={() => {
										meds.splice(index, 1);
										meds = meds;
									}}>Remove</button
								>
							</div>
						{/each}
						<button
							type="button"
							on:click={() => {
								meds = [...meds, ''];
							}}>Add a new medication</button
						>
					{/if}
				</div>
				<button> Submit </button>
			</form>
		{/if}
	{/await}
{/if}

<style>
	form {
		display: flex;
		flex-direction: column;
		gap: 0.5em;
		max-width: 80ch;
		margin: auto;

		& > div {
			display: flex;
			flex-direction: column;
			gap: 0.2em;
		}
	}

	.meds-line {
		display: flex;
		gap: 0.2em;

		& input[type='text'] {
			flex: 1;
		}
	}
</style>
