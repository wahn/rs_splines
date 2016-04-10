import pntvec
import splines

if __name__ == "__main__":
    curve = splines.BezierCurve([pntvec.Point(0,0),
                                 pntvec.Point(1,1),
                                 pntvec.Point(2,0)])
    print(curve)
    acc = 0.01
    print("length(%s) = %s" % (acc, curve.length(acc)))
    acc = 0.001
    print("length(%s) = %s" % (acc, curve.length(acc)))
    curve = splines.BezierCurve([pntvec.Point(0,0),
                                 pntvec.Point(0,1),
                                 pntvec.Point(1,1),
                                 pntvec.Point(1,0)])
    print(curve)
    acc = 0.01
    print("length(%s) = %s" % (acc, curve.length(acc)))
    acc = 0.001
    print("length(%s) = %s" % (acc, curve.length(acc)))
