/*
 * DisplayImage.cpp
 *
 *  Created on: 24 Sep, 2014
 *      Author: zephyr
 */

#include <opencv/cv.h>
#include <opencv/highgui.h>
#include <opencv2/opencv.hpp>
#include <iostream>

using namespace std;
using namespace cv;

int main(int argc, char ** argv) {

	// 检查输入
	if (argc != 2) {
		cout << " Usage: display_image ImageToLoadAndDisplay" << endl;
		return -1;
	}

	// 载入原始图像
	Mat image;
	image = imread(argv[1], CV_LOAD_IMAGE_COLOR);
	if (argc != 2 || !image.data) {
		printf("No image data \n");
		return -1;
	}

	// 创建灰度图像
	Mat gray_image;

	// 颜色转换
	cvtColor(image, gray_image, CV_BGR2GRAY);

	// 存储新建的灰度图像
	imwrite("res/gray_image.jpg", gray_image);

	// 创建图像窗口
	namedWindow("Display Image", CV_WINDOW_AUTOSIZE);
	namedWindow("Gray Image", CV_WINDOW_AUTOSIZE);

	// 显示图像
	imshow("Display Image", image);
	imshow("Gray Image", gray_image);

	// 创建Mat对象
	Mat M(200, 200, CV_8UC3, Scalar(0, 0, 0));
	cout << "M = " << endl << " " << M << endl << endl;
	namedWindow("m", CV_WINDOW_AUTOSIZE);
	imshow("m", M);

	// 创建多维的Mat
	int sz[3] = { 2, 2, 2 };
	Mat L(3, sz, CV_8UC(1), Scalar::all(0));
	cout << "L = " << endl << " " << L << endl << endl;

	// M必须已经分配了矩阵空间
	M.create(4, 4, CV_8UC(2));
	cout << "M = " << endl << " " << M << endl << endl;

	// matlab style
	Mat E = Mat::eye(4, 4, CV_64F);
	cout << "E = " << endl << " " << E << endl << endl;
	Mat O = Mat::ones(2, 2, CV_32F);
	cout << "O = " << endl << " " << O << endl << endl;
	Mat Z = Mat::zeros(3, 3, CV_8UC1);
	cout << "Z = " << endl << " " << Z << endl << endl;

	// 可以自己赋值
	Mat C = (Mat_<double>(3, 3) << 0, -1, 0, -1, 5, -1, 0, -1, 0);
	cout << "C = " << endl << " " << C << endl << endl;

	// 可以复制某行
	Mat RowClone = C.row(1).clone();
	cout << "RowClone = " << endl << " " << RowClone << endl << endl;

	// 赋随机值
	Mat R = Mat(3, 2, CV_8UC3);
	randu(R, Scalar::all(0), Scalar::all(255));

	// 格式输出
	cout << "R (default) = " << endl << R << endl << endl;
//	cout << "R (python) = " << endl << format(R, "python") << endl << endl;
//	cout << "R (csv) = " << endl << format(R,"csv") << endl << endl;
//	cout << "R (csv)= " << endl << format(R,"csv") << endl << endl;
//	cout << "R (numpy)= " << endl << format(R,"numpy" ) << endl << endl;
//	cout << "R (c)= " << endl << format(R,"C") << endl << endl;

	// 点的输出
	Point2f P(5, 1);
	cout << "Point (2D) = " << P << endl << endl;
	Point3f P3f(2, 6, 7);
	cout << "Point (3D) = " << P3f << endl << endl;

	// 向量输出
	vector<float> v;
	v.push_back( (float)CV_PI);
	v.push_back(2);
	v.push_back(3.01f);
	cout << "Vector of floats via Mat = " << Mat(v) << endl << endl;

	vector<Point2f> vPoints(20);
	for (size_t i = 0; i < vPoints.size(); ++i)
	vPoints[i] = Point2f((float)(i * 5), (float)(i % 7));
	cout << "A vector of 2D Points = " << vPoints << endl << endl;


	waitKey(0);
	return 0;
}

