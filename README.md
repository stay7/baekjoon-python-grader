# boj-cli-python

Baekjoon Online Judge 문제의 테스트 케이스 정답과 비교해주는 프로그램 입니다.

<img width="535" alt="Screen Shot 2021-08-12 at 3 33 38 PM" src="https://user-images.githubusercontent.com/62214433/129149500-d1dfb8af-1294-459e-9f7f-ae3d958524fc.png">

## Commands

### get
> 테스트 케이스를 받아옵니다.

`boj start <problem_num>`

### start
> `boj get` 명령의 확장입니다.
> 테스트 케이스를 받아오고, `<problem_num>.py` 파일을 생성합니다.

`boj start <problem_num>`

![boj start mov](https://user-images.githubusercontent.com/62214433/136578048-8139cae2-9955-4c6b-9458-bafb43c4a4aa.gif)

### test
> 테스트케이스를 실행합니다

`boj test <problem_num>`

![boj test_false mov](https://user-images.githubusercontent.com/62214433/136578188-2918e023-642a-4181-bc55-09f83b4484af.gif)


## Issue
- [ ] 테스트 케이스가 이미 존재하는 경우, `get`, `start` 명령을 실행하면 에러가 출력됩니다.
- [ ] 백준 온라인 저지의 테스트 케이스 번호와 저장된 테스트 케이스의 번호가 일치하지 않은 경우가 있습니다.
