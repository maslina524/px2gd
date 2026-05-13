import websocket
import subprocess
import json

ws = websocket.WebSocket()
ws.connect("ws://localhost:1313")

result = subprocess.run(["././target/release/px2gd.exe", "--file", "././niko.png", "--target", "str", "-x", "0", "-y", "0", "-c", "90", "--json", "--stdout"], capture_output=True, text=True)
json_out = json.loads(result.stdout)
if json_out.get("status") == "Err":
    print(f"The program ended with an error: {json_out.get("message")}")

data = {
    "action": "ADD_OBJECTS",
    "objects": json_out.get("message")
}
ws.send(json.dumps(data))

response = ws.recv()
print(f"{response}")

ws.close()