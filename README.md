# Authentication system with Svelte + Rust(actix-web)

## issue 01

postgresql이 기존에 존재했지만 접속, 시작이 동작하지 않았다.

다양한 방법을 시도해보았지만, 역시 원초적인 해결법인 `삭제 && 다시 설치`로 해결했다.

1. PostgreSQL 종료 & 삭제

```Shell
brew services stop postgresql

brew uninstall --force postgresql
```

나의 경우, 디렉토리에 이상하게 postgres라는 폴더도 존재하기에 같이 삭제해줬다.

```Shell
cd /usr/local/var

rm -rf postgres
```

2. 삭제되었는지 확인

`brew list`를 입력했을 때 보이는 목록들에서 postgresql이 안보이면 정상적으로 삭제된 것이다.

```Shell
brew list
```

3. 새로 설치

버전을 강제해준다. 최신은 15지만, 최신은 웬만하면 거르는 게 정신 건강에 이롭기 때문에 14로 설치해준다.

```Shell
brew install postgresql@14
```

4. 시작해보기

```Shell
brew services start postgresql@14

psql
```

이렇게 입력했을 떄, 다음과 같이 뜬다면 성공한 것이다.

```Shell
 ~   main ±✚  psql postgres
psql (14.9 (Homebrew))
Type "help" for help.

postgres=#
postgres=#
```

## issue 02

PostgreSQL에 데이터 베이스 && 로그인이 제대로 설정되지 않음.

먼저, GUI로 편하게 작업하기 위해서 pgAdmin을 설치해준다.

window에서는 설치하자마자 알아서

servers 안에 PostgreSQL 14가 생성되어 있었지만 이상하게 생성되지 않았다.

이를 해결해주기 위해 `servers 우클릭` -> `Register` -> `Server`

<img src="/static/images/screenshot1.png" />

<img src="/static/images/screenshot2.png" />

이제 확인해보면, Login/Group Roles에 default인 postgres & seilylook이 존재한다.

처음 설치할 때, seilylook의 password를 설정해주지 않았기 때문에 password를 설정해준다.

마지막으로 `Databases` 우클릭해서 `Create` -> `Database` 선택해주고 기존에 정해둔 name=`rust_auth_db_dev`, Owner=`seilylook`로 해준다.

이렇게 하면 정상적으로 연결된다.

```Shell
 ~   main ±✚  psql rust_auth_db_dev
psql (14.9 (Homebrew))
Type "help" for help.

rust_auth_db_dev=# \dt
               List of relations
 Schema |       Name       | Type  |   Owner
--------+------------------+-------+-----------
 public | _sqlx_migrations | table | seilylook
 public | user_profile     | table | seilylook
 public | users            | table | seilylook
(3 rows)
```

정상적으로 `user_profile`, `users`가 생성된 것을 볼 수 있다.
