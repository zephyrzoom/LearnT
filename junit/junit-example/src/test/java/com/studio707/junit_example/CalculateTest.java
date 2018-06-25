package com.studio707.junit_example;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

import com.studio707.junit_example.service.Calculate;
import com.studio707.junit_example.service.impl.CalculateImpl;

public class CalculateTest {
	@Test
	public void testAdd() {
		Calculate c = new CalculateImpl();
		int sum = c.add(1, 2);
		assertEquals("相加结果错误", 3, sum);
	}
	
	@Test
	public void testSub() {
		Calculate c = new CalculateImpl();
		int result = c.sub(2, 1);
		assertEquals("相减结果错误", 1, result);
	}
	
	@Test
	public void testMul() {
		Calculate c = new CalculateImpl();
		int result = c.mul(1, 2);
		assertEquals("相乘结果错误", 2, result);
	}
	
	@Test
	public void testDiv() {
		Calculate c = new CalculateImpl();
		int result = c.div(2, 1);
		assertEquals(2, result);
	}
}
