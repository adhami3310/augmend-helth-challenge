# Sample Form - AugMend Health

This was made in the request of AugMend Health as part of their interview process.

This has two components, a RESTful backend written in Rust with Postgres integration and a SvelteKit frontend.

## Run

This is not optimized for production builds. It only serves as a demo. To run this, you have to:

### Run Backend

```bash
cd backend
posgres='YOUR POSTGRESS URL' cargo run
```

### Run Auth Server

```bash
cd frontend
firebase emulators:start
```
### Run Frontend

```bash
cd frontend
npm run dev
```
## Outline

I used this tutorial as a base of integrating firebase: [Simplifying Client-Side Authentication with Firebase and SvelteKit](https://gundogmuseray.medium.com/easy-way-to-stop-worry-about-client-side-auth-with-firebase-and-sveltekit-d17cdcccb663). Then I started developing the backend with neon.tech and Postgres-Tokio using the Actix-Web framework. I included basic testcase for deserializing.

The database uses the following schema:

```sql
CREATE TABLE submissions (uid varchar(128) PRIMARY KEY, json text)
```

I used json as a field to allow for future changes of the form. I think a noSQL database could be more fitting to formdata.

I also used basic authentication flow integrated with firebase, although it's not complete. I decided to use Firebase uid as a form of a unique secure key but didn't necessarily validate it in the backend.