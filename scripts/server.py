from http.server import BaseHTTPRequestHandler, HTTPServer
import time
from collections import deque

hostName = "localhost"
serverPort = 8000

time_diffs = deque([], maxlen=100)

last = 0
bucket = 0
max_bucket = 5000

class MyServer(BaseHTTPRequestHandler):

    def do_GET(self):
        global last
        global bucket

        if time.time() - last > 3:
            bucket = max_bucket
        elif bucket < 0:
            print("ERROR: BUCKET EMPTY!!!")
        last = time.time()

        bucket -= 1

        self.send_response(200)
        t = time.strftime("%a, %d %b %Y %H:%M:%S GMT", time.gmtime())
        self.send_header("Content-type", "text/html")
        self.send_header("X-RateLimit-Remaining", str(bucket))
        self.send_header("X-RateLimit-Limit", str(max_bucket))

        self.send_header("X-RateLimit-Reset", str(int(time.time() + 3)))
        self.send_header("Date", t)
        self.end_headers()
        self.wfile.write(bytes("Hi!", "utf-8"))
        
        time_diffs.append(time.time())

        print(time.time(), len(time_diffs) / (time_diffs[-1] - time_diffs[0]))

if __name__ == "__main__":        

    webServer = HTTPServer((hostName, serverPort), MyServer)
    print("Server started http://%s:%s" % (hostName, serverPort))

    try:
        webServer.serve_forever()
    except KeyboardInterrupt:
        pass

    webServer.server_close()
    print("Server stopped.")