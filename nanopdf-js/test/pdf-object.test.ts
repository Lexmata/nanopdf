import { describe, it, expect } from 'vitest';
import {
  pdfNull,
  pdfBool,
  pdfInt,
  pdfReal,
  pdfString,
  pdfName,
  pdfArray,
  pdfDict,
  PdfArray,
  PdfDict,
  isNull,
  isBool,
  isInt,
  isReal,
  isNumber,
  isName,
  isString,
  isArray,
  isDict,
  pdfObjectCompare,
  pdfNameEquals,
  pdfDeepCopy,
  pdfCopyArray,
  pdfCopyDict,
  toBoolDefault,
  toIntDefault,
  toRealDefault,
} from '../src/index.js';

describe('PDF Object - Basic Types', () => {
  it('creates null object', () => {
    const obj = pdfNull();
    expect(isNull(obj)).toBe(true);
    expect(obj.toBool()).toBe(false);
    expect(obj.toInt()).toBe(0);
  });

  it('creates boolean objects', () => {
    const trueObj = pdfBool(true);
    const falseObj = pdfBool(false);
    
    expect(isBool(trueObj)).toBe(true);
    expect(trueObj.toBool()).toBe(true);
    expect(falseObj.toBool()).toBe(false);
  });

  it('creates integer objects', () => {
    const obj = pdfInt(42);
    expect(isInt(obj)).toBe(true);
    expect(isNumber(obj)).toBe(true);
    expect(obj.toInt()).toBe(42);
    expect(obj.toReal()).toBe(42);
  });

  it('creates real objects', () => {
    const obj = pdfReal(3.14);
    expect(isReal(obj)).toBe(true);
    expect(isNumber(obj)).toBe(true);
    expect(obj.toReal()).toBe(3.14);
    expect(obj.toInt()).toBe(3);
  });

  it('creates string objects', () => {
    const obj = pdfString('hello');
    expect(isString(obj)).toBe(true);
    expect(obj.toString()).toBe('hello');
  });

  it('creates name objects', () => {
    const obj = pdfName('Type');
    expect(isName(obj)).toBe(true);
    expect(obj.toName()).toBe('Type');
    expect(pdfNameEquals(obj, 'Type')).toBe(true);
    expect(pdfNameEquals(obj, 'Other')).toBe(false);
  });
});

describe('PDF Object - Arrays', () => {
  it('creates empty array', () => {
    const arr = pdfArray();
    expect(isArray(arr)).toBe(true);
    expect(arr.length).toBe(0);
  });

  it('creates array with items', () => {
    const arr = pdfArray([pdfInt(1), pdfInt(2), pdfInt(3)]);
    expect(arr.length).toBe(3);
    expect(arr.get(0)?.toInt()).toBe(1);
    expect(arr.get(1)?.toInt()).toBe(2);
    expect(arr.get(2)?.toInt()).toBe(3);
  });

  it('pushes items to array', () => {
    const arr = pdfArray();
    arr.push(pdfInt(1));
    arr.pushInt(2);
    arr.pushReal(3.14);
    arr.pushBool(true);
    arr.pushName('Test');
    arr.pushString('hello');
    
    expect(arr.length).toBe(6);
    expect(arr.get(0)?.toInt()).toBe(1);
    expect(arr.get(1)?.toInt()).toBe(2);
    expect(arr.get(2)?.toReal()).toBe(3.14);
    expect(arr.get(3)?.toBool()).toBe(true);
    expect(arr.get(4)?.toName()).toBe('Test');
    expect(arr.get(5)?.toString()).toBe('hello');
  });

  it('inserts items into array', () => {
    const arr = pdfArray([pdfInt(1), pdfInt(3)]);
    arr.insert(1, pdfInt(2));
    
    expect(arr.length).toBe(3);
    expect(arr.get(0)?.toInt()).toBe(1);
    expect(arr.get(1)?.toInt()).toBe(2);
    expect(arr.get(2)?.toInt()).toBe(3);
  });

  it('deletes items from array', () => {
    const arr = pdfArray([pdfInt(1), pdfInt(2), pdfInt(3)]);
    arr.delete(1);
    
    expect(arr.length).toBe(2);
    expect(arr.get(0)?.toInt()).toBe(1);
    expect(arr.get(1)?.toInt()).toBe(3);
  });

  it('puts items at index', () => {
    const arr = pdfArray([pdfInt(1), pdfInt(2), pdfInt(3)]);
    arr.put(1, pdfInt(42));
    
    expect(arr.get(1)?.toInt()).toBe(42);
  });

  it('tracks dirty state', () => {
    const arr = pdfArray();
    expect(arr.isDirty()).toBe(false);
    
    arr.push(pdfInt(1));
    expect(arr.isDirty()).toBe(true);
    
    arr.markClean();
    expect(arr.isDirty()).toBe(false);
  });

  it('creates shallow copy', () => {
    const original = pdfArray([pdfInt(1), pdfInt(2)]);
    const copy = pdfCopyArray(original);
    
    expect(copy.length).toBe(2);
    expect(copy.get(0)?.toInt()).toBe(1);
    expect(pdfObjectCompare(original, copy)).toBe(true);
  });

  it('creates deep copy', () => {
    const nested = pdfArray([pdfInt(1)]);
    const original = pdfArray([nested]);
    const copy = pdfDeepCopy(original) as PdfArray;
    
    // Modify original nested array
    nested.push(pdfInt(2));
    
    // Copy should not be affected
    const copiedNested = copy.get(0) as PdfArray;
    expect(copiedNested.length).toBe(1);
  });
});

describe('PDF Object - Dictionaries', () => {
  it('creates empty dictionary', () => {
    const dict = pdfDict();
    expect(isDict(dict)).toBe(true);
    expect(dict.length).toBe(0);
  });

  it('creates dictionary with entries', () => {
    const dict = pdfDict({
      'Type': pdfName('Page'),
      'Count': pdfInt(10),
    });
    
    expect(dict.length).toBe(2);
    expect(dict.getName('Type')).toBe('Page');
    expect(dict.getInt('Count')).toBe(10);
  });

  it('puts entries into dictionary', () => {
    const dict = pdfDict();
    dict.put('Key1', pdfString('value1'));
    dict.putInt('Key2', 42);
    dict.putReal('Key3', 3.14);
    dict.putBool('Key4', true);
    dict.putName('Key5', 'Name');
    dict.putString('Key6', 'string');
    
    expect(dict.length).toBe(6);
    expect(dict.getString('Key1')).toBe('value1');
    expect(dict.getInt('Key2')).toBe(42);
    expect(dict.getReal('Key3')).toBe(3.14);
    expect(dict.getBool('Key4')).toBe(true);
    expect(dict.getName('Key5')).toBe('Name');
    expect(dict.getString('Key6')).toBe('string');
  });

  it('deletes entries from dictionary', () => {
    const dict = pdfDict({
      'Key1': pdfInt(1),
      'Key2': pdfInt(2),
    });
    
    dict.del('Key1');
    expect(dict.length).toBe(1);
    expect(dict.has('Key1')).toBe(false);
    expect(dict.has('Key2')).toBe(true);
  });

  it('gets entries with defaults', () => {
    const dict = pdfDict();
    
    expect(dict.getInt('Missing', 42)).toBe(42);
    expect(dict.getReal('Missing', 3.14)).toBe(3.14);
    expect(dict.getBool('Missing', true)).toBe(true);
  });

  it('tracks dirty state', () => {
    const dict = pdfDict();
    expect(dict.isDirty()).toBe(false);
    
    dict.putInt('Key', 1);
    expect(dict.isDirty()).toBe(true);
    
    dict.markClean();
    expect(dict.isDirty()).toBe(false);
  });

  it('creates shallow copy', () => {
    const original = pdfDict({
      'Key1': pdfInt(1),
      'Key2': pdfString('value'),
    });
    const copy = pdfCopyDict(original);
    
    expect(copy.length).toBe(2);
    expect(copy.getInt('Key1')).toBe(1);
    expect(pdfObjectCompare(original, copy)).toBe(true);
  });

  it('creates deep copy', () => {
    const nested = pdfDict({ 'Nested': pdfInt(1) });
    const original = pdfDict({ 'Dict': nested });
    const copy = pdfDeepCopy(original) as PdfDict;
    
    // Modify original nested dict
    nested.putInt('Nested', 2);
    
    // Copy should not be affected
    const copiedNested = copy.getDict('Dict')!;
    expect(copiedNested.getInt('Nested')).toBe(1);
  });

  it('gets keys and values', () => {
    const dict = pdfDict({
      'A': pdfInt(1),
      'B': pdfInt(2),
      'C': pdfInt(3),
    });
    
    const keys = dict.keys();
    expect(keys).toHaveLength(3);
    expect(keys).toContain('A');
    expect(keys).toContain('B');
    expect(keys).toContain('C');
    
    const values = dict.values();
    expect(values).toHaveLength(3);
  });
});

describe('PDF Object - Utility Functions', () => {
  it('compares objects', () => {
    expect(pdfObjectCompare(pdfInt(1), pdfInt(1))).toBe(true);
    expect(pdfObjectCompare(pdfInt(1), pdfInt(2))).toBe(false);
    expect(pdfObjectCompare(pdfString('a'), pdfString('a'))).toBe(true);
    expect(pdfObjectCompare(pdfName('Type'), pdfName('Type'))).toBe(true);
  });

  it('extracts values with defaults', () => {
    expect(toBoolDefault(pdfBool(true), false)).toBe(true);
    expect(toBoolDefault(undefined, true)).toBe(true);
    
    expect(toIntDefault(pdfInt(42), 0)).toBe(42);
    expect(toIntDefault(undefined, 100)).toBe(100);
    
    expect(toRealDefault(pdfReal(3.14), 0)).toBe(3.14);
    expect(toRealDefault(undefined, 2.71)).toBe(2.71);
  });
});
