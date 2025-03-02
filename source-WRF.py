import time
import subprocess
import webbrowser
from http.server import SimpleHTTPRequestHandler
from socketserver import TCPServer

isPageOpened = False

subprocess.Popen(r"redis\redis-server.exe redis.conf",cwd=r"redis")
time.sleep(2)
subprocess.Popen(r"wechat849\main.exe",cwd=r"wechat849")

# 配置端口和目录
PORT = 8001  # 服务监听的端口
DIRECTORY = "web"  # 提供服务的目录

class CustomHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        # 设置工作目录
        kwargs['directory'] = DIRECTORY
        super().__init__(*args, **kwargs)

# 启动 HTTP 服务
with TCPServer(("", PORT), CustomHandler) as httpd:
    print(f"在http://localhost:{PORT}启动web静态文件服务,目录: {DIRECTORY}")
    if not isPageOpened:
            webbrowser.open(f"http://localhost:{PORT}")
            isPageOpened = True
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("正在关闭web静态文件服务")
        httpd.shutdown()


