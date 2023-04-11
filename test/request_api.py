import requests
import time
import subprocess
import os
import tempfile
import shutil

host = "127.0.0.1"
port = "8089"
url = "http://" + host + ":" + port

def ping():
    api = url + "/ping"
    response = requests.get(api)
    print("ping result:", response.text)

def register(username, password):
    api = url + "/api/v0/register"
    response = requests.post(api, json={"username": username, "password": password})
    print("register status: {} result: {}".format(response.status_code, response.text))
    if response.status_code == 200:
        return response.json()['token']

def login(username, password):
    api = url + "/api/v0/login"
    response = requests.post(api, json={"username": username, "password": password})
    print("login status: {} result: {}".format(response.status_code, response.text))
    if response.status_code == 200:
        return response.json()['token']

def send_message(token, send, recv, content):
    api = url + "/api/v0/send"
    headers = {'Authorization': 'Bearer ' + token}
    response = requests.post(api, json={
        "message": {
            "send": send, 
            "recv": recv, 
            "content": content, 
            "timestamp":str(int(time.time()))
        } 
    }, headers=headers)
    print("send status: {} result: {}".format(response.status_code, response.text))

def recv_message(token, since):
    api = url + "/api/v0/recv"
    headers = {'Authorization': 'Bearer ' + token}
    response = requests.post(api, json={"since": since}, headers=headers)
    print("sync status: {} result: {}".format(response.status_code, response.text))
    if response.status_code == 200:
        return response.json()


def test_http():
    ping()
    # two user 
    user0 = "user0"
    user1 = "user1"
    register(user0, "password")
    user1_token = register(user1, "password")
    user0_token = login("user0", "password")

    # user0 -> user1 
    # the message: {"send": "user0", "recv": "user1", "content": "hello"}
    send_message(user0_token, user0, user1, "hellow")

    # user1 get message 
    result = recv_message(user1_token, "0")
    (user1_next_since, user1_messages) = result['next_since'], result['messages']
    assert user1_next_since == "1", "user1 get next_since failed"
    assert len(user1_messages) == 1, "user1 get message size failed"
    assert user1_messages[0]['content'] == "hellow", "user1 get message content failed"


    # user1 -> user0 
    send_message(user1_token, user1, user0, "message0 ")
    send_message(user1_token, user1, user0, "message1 ")
    send_message(user1_token, user1, user0, "message2 ")
    result = recv_message(user0_token, "0")
    (user0_next_since, user0_messages) = result['next_since'], result['messages']
    assert user0_next_since == "4", "user0 get next_since failed"
    assert len(user0_messages) == 4, "user0 get message size failed"
    assert user0_messages[0]['content'] == "hellow", "user1 get message content failed"
    assert user0_messages[1]['content'] == "message0 ", "user0 get message content failed"
    assert user0_messages[2]['content'] == "message1 ", "user0 get message content failed"
    assert user0_messages[3]['content'] == "message2 ", "user0 get message content failed"




def run_server(database_tmp_directory):
    working_directory = "../"

    # build server 
    result = subprocess.run(["cargo", "build"], cwd=working_directory)
    if result.returncode != 0:
        print("cargo build failed")
        return None

    # run server 
    my_env = os.environ.copy()
    my_env["MATRIX_ADDRESS"] = host
    my_env["MATRIX_PORT"] =  port
    my_env["MATRIX_DATABASE_BACKEND"] =  "sqlite" # default
    my_env["MATRIX_DATABASE_PATH"] =  database_tmp_directory
    my_env["MATRIX_CONFIG"] = "config.toml"
    process =  subprocess.Popen(["./target/debug/matrix-server"], cwd=working_directory, env=my_env)
    # wait server setup 
    time.sleep(2)
    return process

def main():
    # Create a temporary directory
    temp_dir = tempfile.mkdtemp()

    # run server 
    print("================ build run server ================")
    process = run_server(temp_dir)
    if process is None or process.poll() is not None:
        print("popen run server faild")
        return

    print(process)

    print("================ test http api    ================")

    try:
        test_http()
        print("================ test ok") 
    except requests.ConnectionError:
        print("connection error")
    except AssertionError as e: 
        print("assert error:", e)


    
    print("close server")
    while process.poll() is None:
        print("Process is still closing...")
        process.terminate()
        time.sleep(1)

    # Delete the temporary directory
    shutil.rmtree(temp_dir)


if __name__ == "__main__":
    main()