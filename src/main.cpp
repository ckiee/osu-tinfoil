#include <Arduino.h>

void loop()
{
	Serial.println(touchRead(4));
}

void setup()
{
	Serial.begin(115200);
	pinMode(4, INPUT_PULLDOWN);
}