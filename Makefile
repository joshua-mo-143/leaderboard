curldev:
	curl -X POST -H 'Content-Type: application/json' http://localhost:8000/submit -d '{"name":"josh", "score":$(SCORE)}'
curlprod:
	curl -X POST -H 'Content-Type: application/json' https://shuttlegame-leaderboard.shuttleapp.rs/submit -d '{"name":"josh", "score":$(SCORE)}'
