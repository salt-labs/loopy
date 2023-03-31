secret/cert-authority unchanged
I0331 07:16:51.892776  485907 round_trippers.go:435] curl -k -v -XGET  -H "Accept: application/json" -H "User-Agent: kubectl/v1.21.0+vmware.wcp.2 (linux/amd64) kubernetes/d5bb178" -H "Authorization: Bearer <masked>" 'https://172.18.18.29:6443/apis/cert-manager.io/v1/clusterissuers/cert-issuer'
I0331 07:16:51.896133  485907 round_trippers.go:454] GET https://172.18.18.29:6443/apis/cert-manager.io/v1/clusterissuers/cert-issuer 404 Not Found in 3 milliseconds
I0331 07:16:51.896178  485907 round_trippers.go:460] Response Headers:
I0331 07:16:51.896217  485907 round_trippers.go:463]     X-Kubernetes-Pf-Flowschema-Uid: 73718bb3-d59b-4106-bb35-4baeaddf1622
I0331 07:16:51.896234  485907 round_trippers.go:463]     X-Kubernetes-Pf-Prioritylevel-Uid: c51647d1-57c7-4c09-8d5f-ca0d8a56f4dd
I0331 07:16:51.896249  485907 round_trippers.go:463]     Content-Length: 252
I0331 07:16:51.896279  485907 round_trippers.go:463]     Date: Fri, 31 Mar 2023 07:16:52 GMT
I0331 07:16:51.896296  485907 round_trippers.go:463]     Audit-Id: 6629a9f4-7a24-48f1-96c7-ffe879b0fa65
I0331 07:16:51.896311  485907 round_trippers.go:463]     Cache-Control: no-cache, private
I0331 07:16:51.896328  485907 round_trippers.go:463]     Content-Type: application/json
I0331 07:16:51.896376  485907 request.go:1123] Response Body: {"kind":"Status","apiVersion":"v1","metadata":{},"status":"Failure","message":"clusterissuers.cert-manager.io \"cert-issuer\" not found","reason":"NotFound","details":{"name":"cert-issuer","group":"cert-manager.io","kind":"clusterissuers"},"code":404}
I0331 07:16:51.897147  485907 request.go:1123] Request Body: {"apiVersion":"cert-manager.io/v1","kind":"ClusterIssuer","metadata":{"annotations":{"kubectl.kubernetes.io/last-applied-configuration":"{\"apiVersion\":\"cert-manager.io/v1\",\"kind\":\"ClusterIssuer\",\"metadata\":{\"annotations\":{},\"name\":\"cert-issuer\"},\"spec\":{\"ca\":{\"crlDistributionPoints\":[\"http://ca.poc.domain_here/certsrv/certcarc.asp\"],\"secretName\":\"cert-authority\"}}}\n"},"name":"cert-issuer"},"spec":{"ca":{"crlDistributionPoints":["http://ca.poc.domain_here/certsrv/certcarc.asp"],"secretName":"cert-authority"}}}
I0331 07:16:51.897397  485907 round_trippers.go:435] curl -k -v -XPOST  -H "User-Agent: kubectl/v1.21.0+vmware.wcp.2 (linux/amd64) kubernetes/d5bb178" -H "Accept: application/json" -H "Authorization: Bearer <masked>" -H "Content-Type: application/json" 'https://172.18.18.29:6443/apis/cert-manager.io/v1/clusterissuers?fieldManager=kubectl-client-side-apply'
I0331 07:16:51.928224  485907 round_trippers.go:454] POST https://172.18.18.29:6443/apis/cert-manager.io/v1/clusterissuers?fieldManager=kubectl-client-side-apply 500 Internal Server Error in 30 milliseconds
I0331 07:16:51.928308  485907 round_trippers.go:460] Response Headers:
I0331 07:16:51.928333  485907 round_trippers.go:463]     Audit-Id: 4e2a3db4-6de0-4514-92c0-f2ea9f3cc801
I0331 07:16:51.928349  485907 round_trippers.go:463]     Cache-Control: no-cache, private
I0331 07:16:51.928359  485907 round_trippers.go:463]     Content-Type: application/json
I0331 07:16:51.928380  485907 round_trippers.go:463]     X-Kubernetes-Pf-Flowschema-Uid: 73718bb3-d59b-4106-bb35-4baeaddf1622
I0331 07:16:51.928401  485907 round_trippers.go:463]     X-Kubernetes-Pf-Prioritylevel-Uid: c51647d1-57c7-4c09-8d5f-ca0d8a56f4dd
I0331 07:16:51.928416  485907 round_trippers.go:463]     Content-Length: 465
I0331 07:16:51.928430  485907 round_trippers.go:463]     Date: Fri, 31 Mar 2023 07:16:52 GMT
I0331 07:16:51.928478  485907 request.go:1123] Response Body: {"kind":"Status","apiVersion":"v1","metadata":{},"status":"Failure","message":"Internal error occurred: failed calling webhook \"webhook.cert-manager.io\": Post \"https://cert-manager-webhook.cert-manager.svc:443/mutate?timeout=10s\": Bad Gateway","reason":"InternalError","details":{"causes":[{"message":"failed calling webhook \"webhook.cert-manager.io\": Post \"https://cert-manager-webhook.cert-manager.svc:443/mutate?timeout=10s\": Bad Gateway"}]},"code":500}
I0331 07:16:51.929009  485907 helpers.go:216] server response object: [{
  "kind": "Status",
  "apiVersion": "v1",
  "metadata": {},
  "status": "Failure",
  "message": "error when creating \"config/manifests/cert-authority/ca.yaml\": Internal error occurred: failed calling webhook \"webhook.cert-manager.io\": Post \"https://cert-manager-webhook.cert-manager.svc:443/mutate?timeout=10s\": Bad Gateway",
  "reason": "InternalError",
  "details": {
    "causes": [
      {
        "message": "failed calling webhook \"webhook.cert-manager.io\": Post \"https://cert-manager-webhook.cert-manager.svc:443/mutate?timeout=10s\": Bad Gateway"
      }
    ]
  },
  "code": 500
}]
F0331 07:16:51.929108  485907 helpers.go:115] Error from server (InternalError): error when creating "config/manifests/cert-authority/ca.yaml": Internal error occurred: failed calling webhook "webhook.cert-manager.io": Post "https://cert-manager-webhook.cert-manager.svc:443/mutate?timeout=10s": Bad Gateway
goroutine 1 [running]:
k8s.io/kubernetes/vendor/k8s.io/klog/v2.stacks(0xc00013e001, 0xc002f46000, 0x134, 0x185)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/k8s.io/klog/v2/klog.go:1021 +0xb9
k8s.io/kubernetes/vendor/k8s.io/klog/v2.(*loggingT).output(0x30554c0, 0xc000000003, 0x0, 0x0, 0xc0005e2150, 0x25f3067, 0xa, 0x73, 0x40e300)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/k8s.io/klog/v2/klog.go:970 +0x191
k8s.io/kubernetes/vendor/k8s.io/klog/v2.(*loggingT).printDepth(0x30554c0, 0xc000000003, 0x0, 0x0, 0x0, 0x0, 0x2, 0xc0005c2500, 0x1, 0x1)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/k8s.io/klog/v2/klog.go:733 +0x16f
k8s.io/kubernetes/vendor/k8s.io/klog/v2.FatalDepth(...)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/k8s.io/klog/v2/klog.go:1495
k8s.io/kubernetes/vendor/k8s.io/kubectl/pkg/cmd/util.fatal(0xc0005e6240, 0x105, 0x1)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/k8s.io/kubectl/pkg/cmd/util/helpers.go:93 +0x288
k8s.io/kubernetes/vendor/k8s.io/kubectl/pkg/cmd/util.checkErr(0x207dea0, 0xc0005ba460, 0x1f064f0)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/k8s.io/kubectl/pkg/cmd/util/helpers.go:188 +0x935
k8s.io/kubernetes/vendor/k8s.io/kubectl/pkg/cmd/util.CheckErr(...)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/k8s.io/kubectl/pkg/cmd/util/helpers.go:115
k8s.io/kubernetes/vendor/k8s.io/kubectl/pkg/cmd/apply.NewCmdApply.func1(0xc000a3d080, 0xc000376c60, 0x0, 0x3)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/k8s.io/kubectl/pkg/cmd/apply/apply.go:180 +0x12b
k8s.io/kubernetes/vendor/github.com/spf13/cobra.(*Command).execute(0xc000a3d080, 0xc000376c30, 0x3, 0x3, 0xc000a3d080, 0xc000376c30)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/github.com/spf13/cobra/command.go:854 +0x2c2
k8s.io/kubernetes/vendor/github.com/spf13/cobra.(*Command).ExecuteC(0xc000410dc0, 0xc000140120, 0xc000144000, 0x5)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/github.com/spf13/cobra/command.go:958 +0x375
k8s.io/kubernetes/vendor/github.com/spf13/cobra.(*Command).Execute(...)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/github.com/spf13/cobra/command.go:895
main.main()
	_output/dockerized/go/src/k8s.io/kubernetes/cmd/kubectl/kubectl.go:49 +0x21d

goroutine 18 [chan receive]:
k8s.io/kubernetes/vendor/k8s.io/klog/v2.(*loggingT).flushDaemon(0x30554c0)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/k8s.io/klog/v2/klog.go:1164 +0x8b
created by k8s.io/kubernetes/vendor/k8s.io/klog/v2.init.0
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/k8s.io/klog/v2/klog.go:418 +0xdf

goroutine 7 [select]:
k8s.io/kubernetes/vendor/k8s.io/apimachinery/pkg/util/wait.BackoffUntil(0x1f06410, 0x207e1a0, 0xc0002d1410, 0x1, 0xc000114b40)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/k8s.io/apimachinery/pkg/util/wait/wait.go:167 +0x118
k8s.io/kubernetes/vendor/k8s.io/apimachinery/pkg/util/wait.JitterUntil(0x1f06410, 0x12a05f200, 0x0, 0x1, 0xc000114b40)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/k8s.io/apimachinery/pkg/util/wait/wait.go:133 +0x98
k8s.io/kubernetes/vendor/k8s.io/apimachinery/pkg/util/wait.Until(0x1f06410, 0x12a05f200, 0xc000114b40)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/k8s.io/apimachinery/pkg/util/wait/wait.go:90 +0x4d
created by k8s.io/kubernetes/vendor/k8s.io/kubectl/pkg/util/logs.InitLogs
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/k8s.io/kubectl/pkg/util/logs/logs.go:51 +0x96

goroutine 23 [IO wait]:
internal/poll.runtime_pollWait(0x7f21866cb4d8, 0x72, 0xffffffffffffffff)
	/usr/local/go/src/runtime/netpoll.go:222 +0x55
internal/poll.(*pollDesc).wait(0xc000198118, 0x72, 0x5700, 0x57c7, 0xffffffffffffffff)
	/usr/local/go/src/internal/poll/fd_poll_runtime.go:87 +0x45
internal/poll.(*pollDesc).waitRead(...)
	/usr/local/go/src/internal/poll/fd_poll_runtime.go:92
internal/poll.(*FD).Read(0xc000198100, 0xc002db6000, 0x57c7, 0x57c7, 0x0, 0x0, 0x0)
	/usr/local/go/src/internal/poll/fd_unix.go:166 +0x1d5
net.(*netFD).Read(0xc000198100, 0xc002db6000, 0x57c7, 0x57c7, 0x577c, 0xc002db6046, 0x5)
	/usr/local/go/src/net/fd_posix.go:55 +0x4f
net.(*conn).Read(0xc000392018, 0xc002db6000, 0x57c7, 0x57c7, 0x0, 0x0, 0x0)
	/usr/local/go/src/net/net.go:183 +0x91
crypto/tls.(*atLeastReader).Read(0xc0005d8048, 0xc002db6000, 0x57c7, 0x57c7, 0x577c, 0xc00007e400, 0x0)
	/usr/local/go/src/crypto/tls/conn.go:776 +0x63
bytes.(*Buffer).ReadFrom(0xc000a5a978, 0x207cb00, 0xc0005d8048, 0x40b985, 0x1bcf0a0, 0x1d89e60)
	/usr/local/go/src/bytes/buffer.go:204 +0xbe
crypto/tls.(*Conn).readFromUntil(0xc000a5a700, 0x207f8e0, 0xc000392018, 0x5, 0xc000392018, 0x1da)
	/usr/local/go/src/crypto/tls/conn.go:798 +0xf3
crypto/tls.(*Conn).readRecordOrCCS(0xc000a5a700, 0x0, 0x0, 0x3)
	/usr/local/go/src/crypto/tls/conn.go:605 +0x115
crypto/tls.(*Conn).readRecord(...)
	/usr/local/go/src/crypto/tls/conn.go:573
crypto/tls.(*Conn).Read(0xc000a5a700, 0xc0001e5000, 0x1000, 0x1000, 0x0, 0x0, 0x0)
	/usr/local/go/src/crypto/tls/conn.go:1276 +0x165
bufio.(*Reader).Read(0xc000368f60, 0xc0008b21f8, 0x9, 0x9, 0x95916b, 0xc000613c78, 0x407005)
	/usr/local/go/src/bufio/bufio.go:227 +0x222
io.ReadAtLeast(0x207c920, 0xc000368f60, 0xc0008b21f8, 0x9, 0x9, 0x9, 0xc0005c2120, 0xe2099b69350f00, 0xc0005c2120)
	/usr/local/go/src/io/io.go:328 +0x87
io.ReadFull(...)
	/usr/local/go/src/io/io.go:347
k8s.io/kubernetes/vendor/golang.org/x/net/http2.readFrameHeader(0xc0008b21f8, 0x9, 0x9, 0x207c920, 0xc000368f60, 0x0, 0x0, 0x0, 0x0)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/golang.org/x/net/http2/frame.go:237 +0x89
k8s.io/kubernetes/vendor/golang.org/x/net/http2.(*Framer).ReadFrame(0xc0008b21c0, 0xc0005c4180, 0x0, 0x0, 0x0)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/golang.org/x/net/http2/frame.go:492 +0xa5
k8s.io/kubernetes/vendor/golang.org/x/net/http2.(*clientConnReadLoop).run(0xc000613fa8, 0x0, 0x0)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/golang.org/x/net/http2/transport.go:1819 +0xd8
k8s.io/kubernetes/vendor/golang.org/x/net/http2.(*ClientConn).readLoop(0xc000001800)
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/golang.org/x/net/http2/transport.go:1741 +0x6f
created by k8s.io/kubernetes/vendor/golang.org/x/net/http2.(*Transport).newClientConn
	/go/src/k8s.io/kubernetes/_output/dockerized/go/src/k8s.io/kubernetes/vendor/golang.org/x/net/http2/transport.go:705 +0x6c5

